use super::Check;
use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        terms::{Clause, Cocase},
        types::Ty,
    },
    typing::{
        check::context::compare_typing_contexts,
        errors::Error,
        symbol_table::{Polarity, SymbolTable},
    },
};
use std::collections::HashSet;

impl Check for Cocase {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Cocase, Error> {
        let name = match expected {
            Ty::Int { .. } => {
                return Err(Error::ExpectedIntForCocase {
                    span: self.span.to_miette(),
                })
            }
            Ty::Decl { name, .. } => name,
        };

        let mut expected_dtors: HashSet<String> = match symbol_table.ty_ctors.get(name) {
            Some((Polarity::Codata, dtors)) => dtors.iter().cloned().collect(),
            Some((Polarity::Data, _)) => {
                return Err(Error::ExpectedDataForCocase {
                    span: self.span.to_miette(),
                    data: name.clone(),
                })
            }
            None => {
                return Err(Error::Undefined {
                    span: self.span.to_miette(),
                    name: name.clone(),
                })
            }
        };

        let mut new_clauses = vec![];
        for cocase in self.cocases.into_iter() {
            if !expected_dtors.remove(&cocase.xtor) {
                return Err(Error::UnexpectedDtorInCocase {
                    span: cocase.span.to_miette(),
                    dtor: cocase.xtor.clone(),
                });
            }
            let (dtor_ctx, dtor_ret_ty) = match symbol_table.dtors.get(&cocase.xtor) {
                None => {
                    return Err(Error::Undefined {
                        span: self.span.to_miette(),
                        name: cocase.xtor.clone(),
                    })
                }
                Some(info) => info,
            };

            compare_typing_contexts(&cocase.span.to_miette(), dtor_ctx, &cocase.context)?;

            let mut new_context = context.clone();
            new_context.append(&mut cocase.context.clone());

            let new_rhs = cocase.rhs.check(symbol_table, &new_context, dtor_ret_ty)?;
            let new_clause = Clause {
                span: cocase.span,
                xtor: cocase.xtor,
                context: dtor_ctx.clone(),
                rhs: new_rhs,
            };
            new_clauses.push(new_clause)
        }

        if !expected_dtors.is_empty() {
            return Err(Error::MissingDtorInCocase {
                span: self.span.to_miette(),
            });
        }
        Ok(Cocase {
            span: self.span,
            cocases: new_clauses,
            ty: Some(expected.clone()),
        })
    }
}

#[cfg(test)]
mod cocase_tests {
    use super::Check;
    use crate::{
        syntax::{
            context::ContextBinding,
            terms::{Clause, Cocase, Lit, Var},
            types::Ty,
        },
        typing::symbol_table::{Polarity, SymbolTable},
    };
    use codespan::Span;

    #[test]
    fn check_lpair() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "LPairIntInt".to_owned(),
            (Polarity::Codata, vec!["Fst".to_owned(), "Snd".to_owned()]),
        );
        symbol_table
            .dtors
            .insert("Fst".to_owned(), (vec![], Ty::mk_int()));
        symbol_table
            .dtors
            .insert("Snd".to_owned(), (vec![], Ty::mk_int()));
        let result = Cocase {
            span: Span::default(),
            cocases: vec![
                Clause {
                    span: Span::default(),
                    xtor: "Fst".to_owned(),
                    context: vec![],
                    rhs: Lit {
                        span: Span::default(),
                        val: 1,
                    }
                    .into(),
                },
                Clause {
                    span: Span::default(),
                    xtor: "Snd".to_owned(),
                    context: vec![],
                    rhs: Lit {
                        span: Span::default(),
                        val: 2,
                    }
                    .into(),
                },
            ],
            ty: None,
        }
        .check(&symbol_table, &vec![], &Ty::mk_decl("LPairIntInt"))
        .unwrap();
        let expected = Cocase {
            span: Span::default(),
            cocases: vec![
                Clause {
                    span: Span::default(),
                    xtor: "Fst".to_owned(),
                    context: vec![],
                    rhs: Lit {
                        span: Span::default(),
                        val: 1,
                    }
                    .into(),
                },
                Clause {
                    span: Span::default(),
                    xtor: "Snd".to_owned(),
                    context: vec![],
                    rhs: Lit {
                        span: Span::default(),
                        val: 2,
                    }
                    .into(),
                },
            ],
            ty: Some(Ty::mk_decl("LPairIntInt")),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_fun() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "FunIntInt".to_owned(),
            (Polarity::Codata, vec!["Ap".to_owned()]),
        );
        symbol_table.dtors.insert(
            "Ap".to_owned(),
            (
                vec![ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_int(),
                }],
                Ty::mk_int(),
            ),
        );
        let result = Cocase {
            span: Span::default(),
            cocases: vec![Clause {
                span: Span::default(),
                xtor: "Ap".to_owned(),
                context: vec![ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_int(),
                }],
                rhs: Var {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: None,
                }
                .into(),
            }],
            ty: None,
        }
        .check(&symbol_table, &vec![], &Ty::mk_decl("FunIntInt"))
        .unwrap();
        let expected = Cocase {
            span: Span::default(),
            cocases: vec![Clause {
                span: Span::default(),
                xtor: "Ap".to_owned(),
                context: vec![ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_int(),
                }],
                rhs: Var {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: Some(Ty::mk_int()),
                }
                .into(),
            }],
            ty: Some(Ty::mk_decl("FunIntInt")),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_cocase_fail() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "FunIntInt".to_owned(),
            (Polarity::Codata, vec!["Ap".to_owned()]),
        );
        symbol_table.dtors.insert(
            "Ap".to_owned(),
            (
                vec![ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_int(),
                }],
                Ty::mk_int(),
            ),
        );

        let result = Cocase {
            span: Span::default(),
            cocases: vec![Clause {
                span: Span::default(),
                xtor: "Ap".to_owned(),
                context: vec![],
                rhs: Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            }],
            ty: None,
        }
        .check(&symbol_table, &vec![], &Ty::mk_decl("ListInt"));
        assert!(result.is_err())
    }
}
