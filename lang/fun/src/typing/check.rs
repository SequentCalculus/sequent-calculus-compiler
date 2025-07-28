//! Defines the [Check] trait
use std::rc::Rc;

use codespan::Span;
use miette::SourceSpan;
use printer::Print;

use crate::{
    parser::util::ToMiette,
    syntax::{
        context::{
            Chirality::{Cns, Prd},
            TypingContext,
        },
        substitution::Substitution,
        terms::Term,
        types::Ty,
    },
};

use super::{errors::Error, symbol_table::SymbolTable};

/// Trait to check types with a given symbol table and context
/// The expected type will be saved within the returned Self
pub trait Check: Sized {
    /// Check the type of self
    fn check(
        self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error>;
}

impl<T: Check + Clone> Check for Rc<T> {
    fn check(
        self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let self_checked = Rc::unwrap_or_clone(self).check(symbol_table, context, expected)?;
        Ok(Rc::new(self_checked))
    }
}

impl<T: Check> Check for Option<T> {
    fn check(
        self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        match self {
            None => Ok(None),
            Some(t) => Ok(Some(t.check(symbol_table, context, expected)?)),
        }
    }
}

/// Checks the arguments for a [constructor][crate::syntax::terms::Constructor] or [destructor][crate::syntax::terms::Destructor]
pub fn check_args(
    span: &SourceSpan,
    symbol_table: &mut SymbolTable,
    context: &TypingContext,
    args: Substitution,
    types: &TypingContext,
) -> Result<Substitution, Error> {
    if types.bindings.len() != args.len() {
        return Err(Error::WrongNumberOfArguments {
            span: *span,
            expected: types.bindings.len(),
            got: args.len(),
        });
    }

    let mut new_subst = vec![];
    for (arg, binding) in args.into_iter().zip(types.bindings.iter()) {
        if binding.chi == Cns {
            match arg {
                Term::XVar(mut variable) => {
                    if variable.chi == Some(Prd) {
                        return Err(Error::ExpectedCovariableGotTerm {
                            span: variable.span.to_miette(),
                        });
                    }

                    let found_ty =
                        context.lookup_covar(&variable.var, &variable.span.to_miette())?;
                    if let Some(ty) = variable.ty {
                        check_equality(&variable.span, symbol_table, &ty, &found_ty)?;
                    };

                    check_equality(&variable.span, symbol_table, &binding.ty, &found_ty)?;

                    variable.ty = Some(found_ty);
                    variable.chi = Some(Cns);
                    new_subst.push(variable.into());
                }
                _ => return Err(Error::ExpectedCovariableGotTerm { span: *span }),
            }
        } else {
            binding.ty.check(&types.span, symbol_table)?;

            let arg_checked = arg.check(symbol_table, context, &binding.ty)?;
            new_subst.push(arg_checked);
        }
    }

    Ok(new_subst)
}

/// Checks equality of two given types (within a given symbol table)
pub fn check_equality(
    span: &Span,
    symbol_table: &mut SymbolTable,
    expected: &Ty,
    got: &Ty,
) -> Result<(), Error> {
    expected.check(span, symbol_table)?;
    got.check(span, symbol_table)?;
    if expected != got {
        return Err(Error::Mismatch {
            span: span.to_miette(),
            expected: expected.print_to_string(None),
            got: got.print_to_string(None),
        });
    }
    Ok(())
}

#[cfg(test)]
mod check_tests {
    use super::{check_args, check_equality};
    use crate::{
        parser::util::ToMiette,
        syntax::{
            context::{
                Chirality::{Cns, Prd},
                ContextBinding, TypingContext,
            },
            declarations::{CheckedModule, Module},
            terms::{Constructor, Lit, XVar},
            types::{Ty, TypeArgs},
        },
        test_common::{
            codata_stream, data_list, data_list_i64, def_mult, def_mult_typed, symbol_table_fun,
            symbol_table_list,
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;

    #[test]
    fn module_check() {
        let result = Module {
            declarations: vec![
                data_list().into(),
                codata_stream().into(),
                def_mult().into(),
            ],
        }
        .check()
        .unwrap();

        let expected = CheckedModule {
            defs: vec![def_mult_typed()],
            data_types: vec![data_list_i64()],
            codata_types: vec![],
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_check_int() {
        let result = Ty::mk_i64().check(&Span::default(), &mut SymbolTable::default());
        assert!(result.is_ok())
    }

    #[test]
    fn ty_check_decl() {
        let mut symbol_table = symbol_table_list();
        let result = Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()]))
            .check(&Span::default(), &mut symbol_table);
        assert!(result.is_ok())
    }

    #[test]
    fn ty_check_fail() {
        let result = Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()]))
            .check(&Span::default(), &mut SymbolTable::default());
        assert!(result.is_err())
    }

    #[test]
    fn equality_check() {
        let result = check_equality(
            &Span::default(),
            &mut SymbolTable::default(),
            &Ty::mk_i64(),
            &Ty::mk_i64(),
        );
        assert!(result.is_ok())
    }

    #[test]
    fn equality_check_fail() {
        let result = check_equality(
            &Span::default(),
            &mut SymbolTable::default(),
            &Ty::mk_i64(),
            &Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
        );
        assert!(result.is_err())
    }

    #[test]
    fn check_arg_list() {
        let mut symbol_table = symbol_table_list();
        let result = check_args(
            &Span::default().to_miette(),
            &mut symbol_table,
            &TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
            vec![
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
                Constructor {
                    span: Span::default(),
                    id: "Nil".to_owned(),
                    args: vec![],
                    ty: None,
                }
                .into(),
            ],
            &TypingContext {
                span: Span::default(),
                bindings: vec![
                    ContextBinding {
                        var: "x".to_owned(),
                        chi: Prd,
                        ty: Ty::mk_i64(),
                    },
                    ContextBinding {
                        var: "xs".to_owned(),
                        chi: Prd,
                        ty: Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
                    },
                ],
            },
        )
        .unwrap();
        let expected = vec![
            Lit {
                span: Span::default(),
                val: 1,
            }
            .into(),
            Constructor {
                span: Span::default(),
                id: "Nil".to_owned(),
                args: vec![],
                ty: Some(Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()]))),
            }
            .into(),
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn check_arg_covar() {
        let result = check_args(
            &Span::default().to_miette(),
            &mut symbol_table_fun(),
            &TypingContext {
                span: Span::default(),
                bindings: vec![
                    ContextBinding {
                        var: "c".to_owned(),
                        chi: Cns,
                        ty: Ty::mk_i64(),
                    },
                    ContextBinding {
                        var: "d".to_owned(),
                        chi: Cns,
                        ty: Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
                    },
                ],
            },
            vec![XVar::mk("c").into(), XVar::mk("d").into()],
            &TypingContext {
                span: Span::default(),
                bindings: vec![
                    ContextBinding {
                        var: "a".to_owned(),
                        chi: Cns,
                        ty: Ty::mk_i64(),
                    },
                    ContextBinding {
                        var: "b".to_owned(),
                        chi: Cns,
                        ty: Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
                    },
                ],
            },
        )
        .unwrap();
        let expected = vec![
            XVar {
                span: Span::default(),
                var: "c".to_owned(),
                ty: Some(Ty::mk_i64()),
                chi: Some(Cns),
            }
            .into(),
            XVar {
                span: Span::default(),
                var: "d".to_owned(),
                ty: Some(Ty::mk_decl(
                    "Fun",
                    TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
                )),
                chi: Some(Cns),
            }
            .into(),
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn check_fail() {
        let result = check_args(
            &Span::default().to_miette(),
            &mut SymbolTable::default(),
            &TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
            vec![
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ],
            &TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
        );
        assert!(result.is_err())
    }
}
