//! This module defines a trait with a method for typechecking.

use std::rc::Rc;

use miette::SourceSpan;
use printer::Print;

use crate::syntax::{
    arguments::Arguments,
    context::{
        Chirality::{Cns, Prd},
        TypingContext,
    },
    terms::Term,
    types::Ty,
};

use super::{errors::Error, symbol_table::SymbolTable};

/// This trait defines a method for typechecking against an expected type. The expected type will
/// be annotated in the checked term.
pub trait Check: Sized {
    /// This method performs typechecking with a given symbol table and typing context against an
    /// expected type. The expected type will be annotated in the checked term.
    /// - `symbol_table` is the symbol table during typechecking.
    /// - `context` is the current typing context containing bindings for the (co)variables in
    ///   scope.
    /// - `expected` is the expected type.
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

/// This function typechecks arguments against a signature, i.e.,
/// against the types in a list of bindings.
/// - `span` is the source location of the arguments.
/// - `symbol_table` is the symbol table during typechecking.
/// - `context` is the current typing context.
/// - `args` are the arguments to check.
/// - `types` is the list of bindings against whose types the arguments are checked.
pub fn check_args(
    span: &SourceSpan,
    symbol_table: &mut SymbolTable,
    context: &TypingContext,
    args: Arguments,
    types: &TypingContext,
) -> Result<Arguments, Error> {
    if types.bindings.len() != args.entries.len() {
        return Err(Error::WrongNumberOfArguments {
            span: *span,
            expected: types.bindings.len(),
            got: args.entries.len(),
        });
    }

    let mut new_args = vec![];
    for (arg, binding) in args.entries.into_iter().zip(types.bindings.iter()) {
        if binding.chi == Cns {
            match arg {
                Term::XVar(mut variable) => {
                    if variable.chi == Some(Prd) {
                        return Err(Error::ExpectedCovariableGotTerm {
                            span: variable.span,
                        });
                    }

                    let found_ty = context.lookup_covar(&variable.var, &variable.span)?;
                    if let Some(ty) = variable.ty {
                        check_equality(&variable.span, symbol_table, &ty, &found_ty)?;
                    };

                    check_equality(&variable.span, symbol_table, &binding.ty, &found_ty)?;

                    variable.ty = Some(found_ty);
                    variable.chi = Some(Cns);
                    new_args.push(variable.into());
                }
                _ => return Err(Error::ExpectedCovariableGotTerm { span: *span }),
            }
        } else {
            binding.ty.check(&types.span, symbol_table)?;

            let arg_checked = arg.check(symbol_table, context, &binding.ty)?;
            new_args.push(arg_checked);
        }
    }

    Ok(new_args.into())
}

/// This function checks equality of two monomorphic types. It also checks the well-formedness of the two
/// types, which creates instances if needed. The two types hence must not be type parameters.
pub fn check_equality(
    span: &SourceSpan,
    symbol_table: &mut SymbolTable,
    expected: &Ty,
    got: &Ty,
) -> Result<(), Error> {
    expected.check(&Some(*span), symbol_table)?;
    got.check(&Some(*span), symbol_table)?;
    if expected != got {
        return Err(Error::Mismatch {
            span: *span,
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
        syntax::{
            context::{
                Chirality::{Cns, Prd},
                ContextBinding, TypingContext,
            },
            names::Ident,
            program::{CheckedProgram, Program},
            terms::{Constructor, Lit, XVar},
            types::{Ty, TypeArgs},
            util::dummy_span,
        },
        test_common::{
            codata_stream, data_list, data_list_i64, def_mult, def_mult_typed, symbol_table_fun,
            symbol_table_list,
        },
        typing::symbol_table::SymbolTable,
    };

    #[test]
    fn module_check() {
        let result = Program {
            declarations: vec![
                data_list().into(),
                codata_stream().into(),
                def_mult().into(),
            ],
        }
        .check()
        .unwrap();

        let expected = CheckedProgram {
            defs: vec![def_mult_typed()],
            data_types: vec![data_list_i64()],
            codata_types: vec![],
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_check_int() {
        let result = Ty::mk_i64().check(&None, &mut SymbolTable::default());
        assert!(result.is_ok())
    }

    #[test]
    fn ty_check_decl() {
        let mut symbol_table = symbol_table_list();
        let result =
            Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])).check(&None, &mut symbol_table);
        assert!(result.is_ok())
    }

    #[test]
    fn ty_check_fail() {
        let result = Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()]))
            .check(&None, &mut SymbolTable::default());
        assert!(result.is_err())
    }

    #[test]
    fn equality_check() {
        let result = check_equality(
            &dummy_span(),
            &mut SymbolTable::default(),
            &Ty::mk_i64(),
            &Ty::mk_i64(),
        );
        assert!(result.is_ok())
    }

    #[test]
    fn equality_check_fail() {
        let result = check_equality(
            &dummy_span(),
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
            &dummy_span(),
            &mut symbol_table,
            &TypingContext {
                span: None,
                bindings: vec![],
            },
            vec![
                Lit {
                    span: dummy_span(),
                    lit: 1,
                }
                .into(),
                Constructor {
                    span: dummy_span(),
                    id: "Nil".to_owned(),
                    args: vec![].into(),
                    ty: None,
                }
                .into(),
            ]
            .into(),
            &TypingContext {
                span: None,
                bindings: vec![
                    ContextBinding {
                        var: Ident {
                            name: "x".to_owned(),
                            id: 0,
                        },
                        chi: Prd,
                        ty: Ty::mk_i64(),
                    },
                    ContextBinding {
                        var: Ident {
                            name: "xs".to_owned(),
                            id: 0,
                        },
                        chi: Prd,
                        ty: Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
                    },
                ],
            },
        )
        .unwrap();
        let expected = vec![
            Lit {
                span: dummy_span(),
                lit: 1,
            }
            .into(),
            Constructor {
                span: dummy_span(),
                id: "Nil".to_owned(),
                args: vec![].into(),
                ty: Some(Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()]))),
            }
            .into(),
        ]
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_arg_covar() {
        let result = check_args(
            &dummy_span(),
            &mut symbol_table_fun(),
            &TypingContext {
                span: None,
                bindings: vec![
                    ContextBinding {
                        var: Ident {
                            name: "c".to_owned(),
                            id: 0,
                        },
                        chi: Cns,
                        ty: Ty::mk_i64(),
                    },
                    ContextBinding {
                        var: Ident {
                            name: "d".to_owned(),
                            id: 0,
                        },
                        chi: Cns,
                        ty: Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
                    },
                ],
            },
            vec![
                XVar::mk(Ident {
                    name: "c".to_string(),
                    id: 0,
                })
                .into(),
                XVar::mk(Ident {
                    name: "d".to_string(),
                    id: 0,
                })
                .into(),
            ]
            .into(),
            &TypingContext {
                span: None,
                bindings: vec![
                    ContextBinding {
                        var: Ident {
                            name: "a".to_owned(),
                            id: 0,
                        },
                        chi: Cns,
                        ty: Ty::mk_i64(),
                    },
                    ContextBinding {
                        var: Ident {
                            name: "b".to_owned(),
                            id: 0,
                        },
                        chi: Cns,
                        ty: Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
                    },
                ],
            },
        )
        .unwrap();
        let expected = vec![
            XVar {
                span: dummy_span(),
                var: Ident {
                    name: "c".to_owned(),
                    id: 0,
                },
                ty: Some(Ty::mk_i64()),
                chi: Some(Cns),
            }
            .into(),
            XVar {
                span: dummy_span(),
                var: Ident {
                    name: "d".to_owned(),
                    id: 0,
                },
                ty: Some(Ty::mk_decl(
                    "Fun",
                    TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
                )),
                chi: Some(Cns),
            }
            .into(),
        ]
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_fail() {
        let result = check_args(
            &dummy_span(),
            &mut SymbolTable::default(),
            &TypingContext {
                span: None,
                bindings: vec![],
            },
            vec![
                Lit {
                    span: dummy_span(),
                    lit: 1,
                }
                .into(),
            ]
            .into(),
            &TypingContext {
                span: None,
                bindings: vec![],
            },
        );
        assert!(result.is_err())
    }
}
