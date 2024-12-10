use std::rc::Rc;

use miette::SourceSpan;
use printer::Print;

use crate::syntax::{
    context::{ContextBinding, TypingContext},
    substitution::{Substitution, SubstitutionBinding},
    types::Ty,
};

use super::{errors::Error, symbol_table::SymbolTable};

pub trait Check: Sized {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error>;
}

impl<T: Check + Clone> Check for Rc<T> {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let self_checked = Rc::unwrap_or_clone(self).check(symbol_table, context, expected)?;
        Ok(Rc::new(self_checked))
    }
}

pub fn check_args(
    span: &SourceSpan,
    symbol_table: &SymbolTable,
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
    for c in args.into_iter().zip(types.bindings.iter()) {
        match c {
            (SubstitutionBinding::TermBinding(term), ContextBinding::TypedVar { ty, .. }) => {
                let term_checked = term.check(symbol_table, context, ty)?;
                new_subst.push(SubstitutionBinding::TermBinding(term_checked));
            }
            (
                SubstitutionBinding::CovarBinding {
                    covar: cov,
                    ty: subst_ty,
                },
                ContextBinding::TypedCovar { ty, .. },
            ) => {
                let found_ty = context.lookup_covar(&cov)?;
                if Some(&found_ty) == subst_ty.as_ref() || subst_ty.is_none() {
                    Ok(())
                } else {
                    Err(Error::Mismatch {
                        span: *span,
                        expected: found_ty.print_to_string(Default::default()),
                        got: subst_ty.unwrap().print_to_string(Default::default()),
                    })
                }?;

                check_equality(span, ty, &found_ty)?;
                new_subst.push(SubstitutionBinding::CovarBinding {
                    covar: cov,
                    ty: Some(found_ty),
                });
            }
            (SubstitutionBinding::CovarBinding { .. }, ContextBinding::TypedVar { .. }) => {
                return Err(Error::ExpectedTermGotCovariable { span: *span })
            }
            (SubstitutionBinding::TermBinding(..), ContextBinding::TypedCovar { .. }) => {
                return Err(Error::ExpectedCovariableGotTerm { span: *span })
            }
        }
    }
    Ok(new_subst)
}

pub fn check_equality(span: &SourceSpan, expected: &Ty, got: &Ty) -> Result<(), Error> {
    if expected != got {
        return Err(Error::Mismatch {
            span: *span,
            expected: expected.print_to_string(Option::default()),
            got: got.print_to_string(Option::default()),
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
            context::{ContextBinding, TypingContext},
            declarations::Module,
            substitution::SubstitutionBinding,
            terms::{Constructor, Lit},
            types::Ty,
        },
        test_common::{codata_stream, data_list, def_mult, def_mult_typed, symbol_table_list},
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

        let expected = Module {
            declarations: vec![
                data_list().into(),
                codata_stream().into(),
                def_mult_typed().into(),
            ],
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_check_int() {
        let result = Ty::mk_int().check(&SymbolTable::default());
        assert!(result.is_ok())
    }

    #[test]
    fn ty_check_decl() {
        let symbol_table = symbol_table_list();
        let result = Ty::mk_decl("ListInt").check(&symbol_table);
        assert!(result.is_ok())
    }
    #[test]
    fn ty_check_fail() {
        let result = Ty::mk_decl("ListInt").check(&SymbolTable::default());
        assert!(result.is_err())
    }
    #[test]
    fn equality_check() {
        let result = check_equality(&Span::default().to_miette(), &Ty::mk_int(), &Ty::mk_int());
        assert!(result.is_ok())
    }
    #[test]
    fn equality_check_fail() {
        let result = check_equality(
            &Span::default().to_miette(),
            &Ty::mk_int(),
            &Ty::mk_decl("ListInt"),
        );
        assert!(result.is_err())
    }

    #[test]
    fn check_arg_list() {
        let symbol_table = symbol_table_list();
        let result = check_args(
            &Span::default().to_miette(),
            &symbol_table,
            &TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
            vec![
                SubstitutionBinding::TermBinding(
                    Lit {
                        span: Span::default(),
                        val: 1,
                    }
                    .into(),
                ),
                SubstitutionBinding::TermBinding(
                    Constructor {
                        span: Span::default(),
                        id: "Nil".to_owned(),
                        args: vec![],
                        ty: None,
                    }
                    .into(),
                ),
            ],
            &TypingContext {
                span: Span::default(),
                bindings: vec![
                    ContextBinding::TypedVar {
                        var: "x".to_owned(),
                        ty: Ty::mk_int(),
                    },
                    ContextBinding::TypedVar {
                        var: "xs".to_owned(),
                        ty: Ty::mk_decl("ListInt"),
                    },
                ],
            },
        )
        .unwrap();
        let expected = vec![
            SubstitutionBinding::TermBinding(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
            SubstitutionBinding::TermBinding(
                Constructor {
                    span: Span::default(),
                    id: "Nil".to_owned(),
                    args: vec![],
                    ty: Some(Ty::mk_decl("ListInt")),
                }
                .into(),
            ),
        ];
        assert_eq!(result, expected)
    }
    #[test]
    fn check_arg_covar() {
        let result = check_args(
            &Span::default().to_miette(),
            &SymbolTable::default(),
            &TypingContext {
                span: Span::default(),
                bindings: vec![
                    ContextBinding::TypedCovar {
                        covar: "c".to_owned(),
                        ty: Ty::mk_int(),
                    },
                    ContextBinding::TypedCovar {
                        covar: "d".to_owned(),
                        ty: Ty::mk_decl("FunIntInt"),
                    },
                ],
            },
            vec![
                SubstitutionBinding::CovarBinding {
                    covar: "c".to_owned(),
                    ty: None,
                },
                SubstitutionBinding::CovarBinding {
                    covar: "d".to_owned(),
                    ty: None,
                },
            ],
            &TypingContext {
                span: Span::default(),
                bindings: vec![
                    ContextBinding::TypedCovar {
                        covar: "a".to_owned(),
                        ty: Ty::mk_int(),
                    },
                    ContextBinding::TypedCovar {
                        covar: "b".to_owned(),
                        ty: Ty::mk_decl("FunIntInt"),
                    },
                ],
            },
        )
        .unwrap();
        let expected = vec![
            SubstitutionBinding::CovarBinding {
                covar: "c".to_owned(),
                ty: Some(Ty::mk_int()),
            },
            SubstitutionBinding::CovarBinding {
                covar: "d".to_owned(),
                ty: Some(Ty::mk_decl("FunIntInt")),
            },
        ];
        assert_eq!(result, expected)
    }
    #[test]
    fn check_fail() {
        let result = check_args(
            &Span::default().to_miette(),
            &SymbolTable::default(),
            &TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
            vec![SubstitutionBinding::TermBinding(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            )],
            &TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
        );
        assert!(result.is_err())
    }
}
