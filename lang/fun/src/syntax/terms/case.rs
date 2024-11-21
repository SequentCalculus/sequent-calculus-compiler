use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{CASE, DOT},
    DocAllocator, Print,
};

use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        print_cases,
        types::{OptTyped, Ty},
        Name,
    },
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use super::{Clause, Term};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Case {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub destructee: Rc<Term>,
    pub cases: Vec<Clause<Name>>,
    pub ty: Option<Ty>,
}

impl OptTyped for Case {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Case {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.destructee
            .print(cfg, alloc)
            .append(DOT)
            .append(alloc.keyword(CASE))
            .append(alloc.space())
            .append(print_cases(&self.cases, cfg, alloc))
    }
}

impl From<Case> for Term {
    fn from(value: Case) -> Self {
        Term::Case(value)
    }
}

impl Check for Case {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        // Find out the type on which we pattern match by inspecting the first case.
        // We throw an error for empty cases.
        let (ty, mut expected_ctors) = match self.cases.first() {
            Some(case) => symbol_table.lookup_ty_for_ctor(&self.span.to_miette(), &case.xtor)?,
            None => {
                return Err(Error::EmptyMatch {
                    span: self.span.to_miette(),
                })
            }
        };

        // We check the "e" in "case e of {...}" against this type.
        let destructee_checked = self.destructee.check(symbol_table, context, &ty)?;

        let mut new_cases = vec![];
        for case in self.cases {
            if !expected_ctors.remove(&case.xtor) {
                return Err(Error::UnexpectedCtorInCase {
                    span: case.span.to_miette(),
                    ctor: case.xtor.clone(),
                });
            }
            match symbol_table.ctors.get(&case.xtor) {
                Some(ctor_ctx) => {
                    case.context.compare_to(&case.span.to_miette(), ctor_ctx)?;

                    let mut new_context = context.clone();
                    new_context
                        .bindings
                        .append(&mut case.context.bindings.clone());

                    let new_rhs = case.rhs.check(symbol_table, &new_context, expected)?;
                    new_cases.push(Clause {
                        rhs: new_rhs,
                        ..case
                    });
                }
                None => {
                    return Err(Error::Undefined {
                        span: case.span.to_miette(),
                        name: case.xtor.clone(),
                    })
                }
            }
        }
        if !expected_ctors.is_empty() {
            return Err(Error::MissingCtorsInCase {
                span: self.span.to_miette(),
            });
        }
        Ok(Case {
            destructee: destructee_checked,
            cases: new_cases,
            ty: Some(expected.clone()),
            ..self
        })
    }
}

#[cfg(test)]
mod test {
    use super::Check;
    use super::Term;
    use crate::parser::fun;
    use crate::syntax::context::TypingContext;
    use crate::{
        syntax::{
            context::ContextBinding,
            terms::{Case, Clause, Lit, Var},
            types::Ty,
        },
        typing::symbol_table::{Polarity, SymbolTable},
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_case_list() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
        symbol_table
            .ctors
            .insert("Nil".to_owned(), TypingContext { bindings: vec![] });
        symbol_table.ctors.insert(
            "Cons".to_owned(),
            TypingContext {
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
        );
        let result = Case {
            span: Span::default(),
            cases: vec![
                Clause {
                    span: Span::default(),
                    xtor: "Nil".to_owned(),
                    context: TypingContext { bindings: vec![] },
                    rhs: Lit::mk(1).into(),
                },
                Clause {
                    span: Span::default(),
                    xtor: "Cons".to_owned(),
                    context: TypingContext {
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
                    rhs: Var::mk("x").into(),
                },
            ],
            destructee: Rc::new(Var::mk("x").into()),
            ty: None,
        }
        .check(
            &symbol_table,
            &TypingContext {
                bindings: vec![ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_decl("ListInt"),
                }],
            },
            &Ty::mk_int(),
        )
        .unwrap();
        let expected = Case {
            span: Span::default(),
            cases: vec![
                Clause {
                    span: Span::default(),
                    xtor: "Nil".to_owned(),
                    context: TypingContext { bindings: vec![] },
                    rhs: Lit::mk(1).into(),
                },
                Clause {
                    span: Span::default(),
                    xtor: "Cons".to_owned(),
                    context: TypingContext {
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
                    rhs: Var {
                        span: Span::default(),
                        var: "x".to_owned(),
                        ty: Some(Ty::mk_int()),
                    }
                    .into(),
                },
            ],
            destructee: Rc::new(
                Var {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: Some(Ty::mk_decl("ListInt")),
                }
                .into(),
            ),
            ty: Some(Ty::mk_int()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_case_tup() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "TupIntInt".to_owned(),
            (Polarity::Data, vec!["Tup".to_owned()]),
        );
        symbol_table.ctors.insert(
            "Tup".to_owned(),
            TypingContext {
                bindings: vec![
                    ContextBinding::TypedVar {
                        var: "x".to_owned(),
                        ty: Ty::mk_int(),
                    },
                    ContextBinding::TypedVar {
                        var: "y".to_owned(),
                        ty: Ty::mk_int(),
                    },
                ],
            },
        );
        let result = Case {
            span: Span::default(),
            cases: vec![Clause {
                span: Span::default(),
                xtor: "Tup".to_owned(),
                context: TypingContext {
                    bindings: vec![
                        ContextBinding::TypedVar {
                            var: "x".to_owned(),
                            ty: Ty::mk_int(),
                        },
                        ContextBinding::TypedVar {
                            var: "y".to_owned(),
                            ty: Ty::mk_int(),
                        },
                    ],
                },
                rhs: Var::mk("x").into(),
            }],
            destructee: Rc::new(Var::mk("x").into()),
            ty: None,
        }
        .check(
            &symbol_table,
            &TypingContext {
                bindings: vec![ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_decl("TupIntInt"),
                }],
            },
            &Ty::mk_int(),
        )
        .unwrap();
        let expected = Case {
            span: Span::default(),
            cases: vec![Clause {
                span: Span::default(),
                xtor: "Tup".to_owned(),
                context: TypingContext {
                    bindings: vec![
                        ContextBinding::TypedVar {
                            var: "x".to_owned(),
                            ty: Ty::mk_int(),
                        },
                        ContextBinding::TypedVar {
                            var: "y".to_owned(),
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
            destructee: Rc::new(
                Var {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: Some(Ty::mk_decl("TupIntInt")),
                }
                .into(),
            ),
            ty: Some(Ty::mk_int()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_case_fail() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
        symbol_table
            .ctors
            .insert("Nil".to_owned(), TypingContext { bindings: vec![] });
        symbol_table.ctors.insert(
            "Cons".to_owned(),
            TypingContext {
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
        );
        let result = Case {
            span: Span::default(),
            cases: vec![Clause {
                span: Span::default(),
                xtor: "Tup".to_owned(),
                context: TypingContext {
                    bindings: vec![
                        ContextBinding::TypedVar {
                            var: "x".to_owned(),
                            ty: Ty::mk_int(),
                        },
                        ContextBinding::TypedVar {
                            var: "y".to_owned(),
                            ty: Ty::mk_int(),
                        },
                    ],
                },
                rhs: Var::mk("x").into(),
            }],
            destructee: Rc::new(Lit::mk(1).into()),
            ty: None,
        }
        .check(
            &symbol_table,
            &TypingContext { bindings: vec![] },
            &Ty::mk_int(),
        );
        assert!(result.is_err())
    }

    fn example_empty() -> Case {
        Case {
            span: Span::default(),
            destructee: Rc::new(Var::mk("x").into()),
            cases: vec![],
            ty: None,
        }
    }

    fn example_tup() -> Case {
        Case {
            span: Span::default(),
            destructee: Rc::new(Var::mk("x").into()),
            cases: vec![Clause {
                span: Span::default(),
                xtor: "Tup".to_owned(),
                context: TypingContext {
                    bindings: vec![
                        ContextBinding::TypedVar {
                            var: "x".to_string(),
                            ty: Ty::mk_int(),
                        },
                        ContextBinding::TypedVar {
                            var: "y".to_string(),
                            ty: Ty::mk_int(),
                        },
                    ],
                },
                rhs: Term::Lit(Lit::mk(2)),
            }],
            ty: None,
        }
    }

    #[test]
    fn display_empty() {
        assert_eq!(
            example_empty().print_to_string(Default::default()),
            "x.case { }"
        )
    }

    #[test]
    fn parse_empty() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("x.case { }"), Ok(example_empty().into()));
    }

    #[test]
    fn display_tup() {
        assert_eq!(
            example_tup().print_to_string(Default::default()),
            "x.case { Tup(x: Int, y: Int) => 2 }"
        )
    }

    #[test]
    fn parse_tup() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("x.case { Tup(x : Int, y : Int) => 2 }"),
            Ok(example_tup().into())
        );
    }
}
