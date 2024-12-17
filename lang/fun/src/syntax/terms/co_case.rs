use std::{collections::HashSet, rc::Rc};

use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{CASE, COCASE, COMMA, DOT, FAT_ARROW},
    util::BracesExt,
    Alloc, Builder, DocAllocator, Print, PrintCfg,
};

use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        types::{OptTyped, Ty},
        Name,
    },
    typing::{
        check::Check,
        errors::Error,
        symbol_table::{Polarity, SymbolTable},
    },
};

use super::Term;

// Clause
//
//

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Clause {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// Whether we have a clause of a case expression or a co-clause of a cocase expression.
    pub is_clause: bool,
    pub xtor: Name,
    pub context: TypingContext,
    pub rhs: Term,
}

impl OptTyped for Clause {
    fn get_type(&self) -> Option<Ty> {
        self.rhs.get_type()
    }
}

impl Print for Clause {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let xtor = if self.is_clause {
            alloc.ctor(&self.xtor)
        } else {
            alloc.dtor(&self.xtor)
        };
        xtor.append(self.context.print(cfg, alloc))
            .append(alloc.space())
            .append(FAT_ARROW)
            .append(alloc.space())
            .append(self.rhs.print(cfg, alloc))
    }
}

fn print_clauses<'a>(cases: &'a [Clause], cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
    match cases.len() {
        0 => alloc.space().braces_anno(),

        1 => alloc
            .line()
            .append(cases[0].print(cfg, alloc))
            .nest(cfg.indent)
            .append(alloc.line())
            .braces_anno()
            .group(),
        _ => {
            let sep = alloc.text(COMMA).append(alloc.hardline());
            alloc
                .hardline()
                .append(alloc.intersperse(cases.iter().map(|x| x.print(cfg, alloc)), sep.clone()))
                .nest(cfg.indent)
                .append(alloc.hardline())
                .braces_anno()
        }
    }
}

// Case
//
//

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Case {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub destructee: Rc<Term>,
    pub cases: Vec<Clause>,
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
            .append(print_clauses(&self.cases, cfg, alloc))
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
                    case.context.no_dups(case.xtor.clone())?;
                    case.context.compare_to(ctor_ctx)?;

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

// Cocase
//
//

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Cocase {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub cocases: Vec<Clause>,
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
            .append(print_clauses(&self.cocases, cfg, alloc))
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
            Ty::I64 { .. } => {
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
        syntax::context::TypingContext,
        syntax::{
            terms::{Case, Clause, Lit, Var},
            types::Ty,
        },
        test_common::symbol_table_list,
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_case_list() {
        let mut ctx_case = TypingContext::default();
        ctx_case.add_var("x", Ty::mk_i64());
        ctx_case.add_var("xs", Ty::mk_decl("ListInt"));
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_decl("ListInt"));
        let symbol_table = symbol_table_list();
        let result = Case {
            span: Span::default(),
            cases: vec![
                Clause {
                    span: Span::default(),
                    is_clause: true,
                    xtor: "Nil".to_owned(),
                    context: TypingContext::default(),
                    rhs: Lit::mk(1).into(),
                },
                Clause {
                    span: Span::default(),
                    is_clause: true,
                    xtor: "Cons".to_owned(),
                    context: ctx_case.clone(),
                    rhs: Var::mk("x").into(),
                },
            ],
            destructee: Rc::new(Var::mk("x").into()),
            ty: None,
        }
        .check(&symbol_table, &ctx, &Ty::mk_i64())
        .unwrap();
        let expected = Case {
            span: Span::default(),
            cases: vec![
                Clause {
                    span: Span::default(),
                    is_clause: true,
                    xtor: "Nil".to_owned(),
                    context: TypingContext::default(),
                    rhs: Lit::mk(1).into(),
                },
                Clause {
                    span: Span::default(),
                    is_clause: true,
                    xtor: "Cons".to_owned(),
                    context: ctx_case,
                    rhs: Var {
                        span: Span::default(),
                        var: "x".to_owned(),
                        ty: Some(Ty::mk_i64()),
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
            ty: Some(Ty::mk_i64()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_case_fail() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        ctx.add_var("y", Ty::mk_i64());
        let symbol_table = symbol_table_list();
        let result = Case {
            span: Span::default(),
            cases: vec![Clause {
                span: Span::default(),
                is_clause: true,
                xtor: "Tup".to_owned(),
                context: ctx,
                rhs: Var::mk("x").into(),
            }],
            destructee: Rc::new(Lit::mk(1).into()),
            ty: None,
        }
        .check(&symbol_table, &TypingContext::default(), &Ty::mk_i64());
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
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        ctx.add_var("y", Ty::mk_i64());
        Case {
            span: Span::default(),
            destructee: Rc::new(Var::mk("x").into()),
            cases: vec![Clause {
                span: Span::default(),
                is_clause: true,
                xtor: "Tup".to_owned(),
                context: ctx,
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
            "x.case { Tup(x: i64, y: i64) => 2 }"
        )
    }

    #[test]
    fn parse_tup() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("x.case { Tup(x : i64, y : i64) => 2 }"),
            Ok(example_tup().into())
        );
    }
}

#[cfg(test)]
mod test2 {
    use super::{Check, Term};
    use crate::{
        parser::fun,
        syntax::{
            context::TypingContext,
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
                    is_clause: false,
                    xtor: "Fst".to_owned(),
                    context: TypingContext::default(),
                    rhs: Lit::mk(1).into(),
                },
                Clause {
                    span: Span::default(),
                    is_clause: false,
                    xtor: "Snd".to_owned(),
                    context: TypingContext::default(),
                    rhs: Lit::mk(2).into(),
                },
            ],
            ty: None,
        }
        .check(
            &symbol_table,
            &TypingContext::default(),
            &Ty::mk_decl("LPairIntInt"),
        )
        .unwrap();
        let expected = Cocase {
            span: Span::default(),
            cocases: vec![
                Clause {
                    span: Span::default(),
                    is_clause: false,
                    xtor: "Fst".to_owned(),
                    context: TypingContext::default(),
                    rhs: Lit::mk(1).into(),
                },
                Clause {
                    span: Span::default(),
                    is_clause: false,
                    xtor: "Snd".to_owned(),
                    context: TypingContext::default(),
                    rhs: Lit::mk(2).into(),
                },
            ],
            ty: Some(Ty::mk_decl("LPairIntInt")),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_fun() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        ctx.add_covar("a", Ty::mk_i64());
        let symbol_table = symbol_table_fun();
        let result = Cocase {
            span: Span::default(),
            cocases: vec![Clause {
                span: Span::default(),
                is_clause: false,
                xtor: "Ap".to_owned(),
                context: ctx.clone(),
                rhs: Var::mk("x").into(),
            }],
            ty: None,
        }
        .check(
            &symbol_table,
            &TypingContext::default(),
            &Ty::mk_decl("FunIntInt"),
        )
        .unwrap();
        let expected = Cocase {
            span: Span::default(),
            cocases: vec![Clause {
                span: Span::default(),
                is_clause: false,
                xtor: "Ap".to_owned(),
                context: ctx,
                rhs: Var {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: Some(Ty::mk_i64()),
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
                is_clause: false,
                xtor: "Ap".to_owned(),
                context: TypingContext::default(),
                rhs: Lit::mk(1).into(),
            }],
            ty: None,
        }
        .check(
            &symbol_table,
            &TypingContext::default(),
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
                    is_clause: false,
                    xtor: "Hd".to_owned(),
                    context: TypingContext::default(),
                    rhs: Term::Lit(Lit::mk(2)),
                },
                Clause {
                    span: Span::default(),
                    is_clause: false,
                    xtor: "Tl".to_owned(),
                    context: TypingContext::default(),
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
