use std::collections::HashSet;

use codespan::Span;
use derivative::Derivative;
use printer::{theme::ThemeExt, tokens::COCASE, DocAllocator, Print};

use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        print_cases,
        types::{OptTyped, Ty},
        Name,
    },
    typing::{
        check::Check,
        errors::Error,
        symbol_table::{Polarity, SymbolTable},
    },
};

use super::{Clause, Term};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Cocase {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub cocases: Vec<Clause<Name>>,
    pub ty: Option<Ty>,
}

impl OptTyped for Cocase {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Cocase {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(COCASE)
            .append(alloc.space())
            .append(print_cases(&self.cocases, cfg, alloc))
    }
}

impl From<Cocase> for Term {
    fn from(value: Cocase) -> Self {
        Term::Cocase(value)
    }
}

impl Check for Cocase {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
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

        let mut new_cocases = vec![];
        for cocase in self.cocases {
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
            cocase.context.no_dups(cocase.xtor.clone())?;
            cocase.context.compare_to(dtor_ctx)?;

            let mut new_context = context.clone();
            new_context
                .bindings
                .append(&mut cocase.context.bindings.clone());

            let new_rhs = cocase.rhs.check(symbol_table, &new_context, dtor_ret_ty)?;
            new_cocases.push(Clause {
                rhs: new_rhs,
                ..cocase
            });
        }

        if !expected_dtors.is_empty() {
            return Err(Error::MissingDtorInCocase {
                span: self.span.to_miette(),
            });
        }
        Ok(Cocase {
            cocases: new_cocases,
            ty: Some(expected.clone()),
            ..self
        })
    }
}

#[cfg(test)]
mod test {
    use super::{Check, Term};
    use crate::{
        parser::fun,
        syntax::{
            context::{ContextBinding, TypingContext},
            terms::{Clause, Cocase, Lit, Var},
            types::Ty,
        },
        test_common::{symbol_table_fun, symbol_table_lpair},
    };
    use codespan::Span;
    use printer::Print;

    #[test]
    fn check_lpair() {
        let symbol_table = symbol_table_lpair();
        let result = Cocase {
            span: Span::default(),
            cocases: vec![
                Clause {
                    span: Span::default(),
                    xtor: "Fst".to_owned(),
                    context: TypingContext {
                        span: Span::default(),
                        bindings: vec![],
                    },
                    rhs: Lit::mk(1).into(),
                },
                Clause {
                    span: Span::default(),
                    xtor: "Snd".to_owned(),
                    context: TypingContext {
                        span: Span::default(),
                        bindings: vec![],
                    },
                    rhs: Lit::mk(2).into(),
                },
            ],
            ty: None,
        }
        .check(
            &symbol_table,
            &TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
            &Ty::mk_decl("LPairIntInt"),
        )
        .unwrap();
        let expected = Cocase {
            span: Span::default(),
            cocases: vec![
                Clause {
                    span: Span::default(),
                    xtor: "Fst".to_owned(),
                    context: TypingContext {
                        span: Span::default(),
                        bindings: vec![],
                    },
                    rhs: Lit::mk(1).into(),
                },
                Clause {
                    span: Span::default(),
                    xtor: "Snd".to_owned(),
                    context: TypingContext {
                        span: Span::default(),
                        bindings: vec![],
                    },
                    rhs: Lit::mk(2).into(),
                },
            ],
            ty: Some(Ty::mk_decl("LPairIntInt")),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_fun() {
        let symbol_table = symbol_table_fun();
        let result = Cocase {
            span: Span::default(),
            cocases: vec![Clause {
                span: Span::default(),
                xtor: "Ap".to_owned(),
                context: TypingContext {
                    span: Span::default(),
                    bindings: vec![
                        ContextBinding::TypedVar {
                            var: "x".to_owned(),
                            ty: Ty::mk_int(),
                        },
                        ContextBinding::TypedCovar {
                            covar: "a".to_owned(),
                            ty: Ty::mk_int(),
                        },
                    ],
                },
                rhs: Var::mk("x").into(),
            }],
            ty: None,
        }
        .check(
            &symbol_table,
            &TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
            &Ty::mk_decl("FunIntInt"),
        )
        .unwrap();
        let expected = Cocase {
            span: Span::default(),
            cocases: vec![Clause {
                span: Span::default(),
                xtor: "Ap".to_owned(),
                context: TypingContext {
                    span: Span::default(),
                    bindings: vec![
                        ContextBinding::TypedVar {
                            var: "x".to_owned(),
                            ty: Ty::mk_int(),
                        },
                        ContextBinding::TypedCovar {
                            covar: "a".to_owned(),
                            ty: Ty::mk_int(),
                        },
                    ],
                },
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
        let symbol_table = symbol_table_fun();
        let result = Cocase {
            span: Span::default(),
            cocases: vec![Clause {
                span: Span::default(),
                xtor: "Ap".to_owned(),
                context: TypingContext {
                    span: Span::default(),
                    bindings: vec![],
                },
                rhs: Lit::mk(1).into(),
            }],
            ty: None,
        }
        .check(
            &symbol_table,
            &TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
            &Ty::mk_decl("ListInt"),
        );
        assert!(result.is_err())
    }

    fn example_empty() -> Cocase {
        Cocase {
            span: Span::default(),
            cocases: vec![],
            ty: None,
        }
    }

    fn example_stream() -> Cocase {
        Cocase {
            span: Span::default(),
            cocases: vec![
                Clause {
                    span: Span::default(),
                    xtor: "Hd".to_owned(),
                    context: TypingContext {
                        span: Span::default(),
                        bindings: vec![],
                    },
                    rhs: Term::Lit(Lit::mk(2)),
                },
                Clause {
                    span: Span::default(),
                    xtor: "Tl".to_owned(),
                    context: TypingContext {
                        span: Span::default(),
                        bindings: vec![],
                    },
                    rhs: Term::Lit(Lit::mk(4)),
                },
            ],
            ty: None,
        }
    }

    #[test]
    fn display_empty() {
        assert_eq!(
            example_empty().print_to_string(Default::default()),
            "cocase { }"
        )
    }

    #[test]
    fn parse_empty() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("cocase { }"), Ok(example_empty().into()));
    }

    #[test]
    fn display_stream() {
        assert_eq!(
            example_stream().print_to_string(Default::default()),
            "cocase {\n    Hd => 2,\n    Tl => 4\n}"
        )
    }

    #[test]
    fn parse_stream() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("cocase { Hd => 2, Tl => 4 }"),
            Ok(example_stream().into())
        );
    }
}
