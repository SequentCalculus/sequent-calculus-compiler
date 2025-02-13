use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{CASE, COCASE, DOT},
    DocAllocator, Print,
};

use super::{print_clauses, Clause, Term};
use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        declarations::Polarity,
        types::{OptTyped, Ty, TypeArgs},
        used_binders::UsedBinders,
        Var,
    },
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use std::{collections::HashSet, rc::Rc};

// Case
//
//

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Case {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub destructee: Rc<Term>,
    pub type_args: TypeArgs,
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
            .append(self.type_args.print(cfg, alloc))
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
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        // Find out the type on which we pattern match by inspecting the first case.
        // We throw an error for empty cases.
        let (ty, mut expected_ctors) = match self.cases.first() {
            Some(case) => {
                let ctor_name = case.xtor.clone() + &self.type_args.print_to_string(None);
                match symbol_table.lookup_ty_for_ctor(&self.span.to_miette(), &ctor_name) {
                    Ok(ty) => ty,
                    Err(_) => {
                        symbol_table.lookup_ty_template_for_ctor(&case.xtor, &self.type_args)?
                    }
                }
            }
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
            let ctor_name = case.xtor.clone() + &self.type_args.print_to_string(None);
            if !expected_ctors.remove(&ctor_name) {
                return Err(Error::UnexpectedCtorInCase {
                    span: case.span.to_miette(),
                    ctor: ctor_name.clone(),
                });
            }
            match symbol_table.ctors.get(&ctor_name) {
                None => {
                    return Err(Error::Undefined {
                        span: case.span.to_miette(),
                        name: ctor_name.clone(),
                    })
                }
                Some(signature) => {
                    case.context_names.no_dups(&ctor_name)?;
                    let context_clause = case.context_names.add_types(signature)?;

                    let mut new_context = context.clone();
                    new_context
                        .bindings
                        .append(&mut context_clause.bindings.clone());

                    let new_rhs = case.rhs.check(symbol_table, &new_context, expected)?;
                    new_cases.push(Clause {
                        rhs: new_rhs,
                        context: context_clause,
                        ..case
                    });
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

impl UsedBinders for Case {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.destructee.used_binders(used);
        self.cases.used_binders(used);
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
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let (name, type_args) = match expected {
            Ty::I64 { .. } => {
                return Err(Error::ExpectedI64ForCocase {
                    span: self.span.to_miette(),
                })
            }
            Ty::Decl {
                name, type_args, ..
            } => (name, type_args),
        };

        let type_name = name.clone() + &type_args.print_to_string(None);
        let mut expected_dtors: HashSet<String> = match symbol_table.types.get(&type_name) {
            Some((Polarity::Codata, type_args, dtors)) => dtors
                .iter()
                .map(|dtor| dtor.clone() + &type_args.print_to_string(None))
                .collect(),
            Some((Polarity::Data, _, _)) => {
                return Err(Error::ExpectedDataForCocase {
                    span: self.span.to_miette(),
                    data: type_name,
                })
            }
            None => {
                return Err(Error::Undefined {
                    span: self.span.to_miette(),
                    name: type_name,
                })
            }
        };

        let mut new_cocases = vec![];
        for cocase in self.cocases {
            let dtor_name = cocase.xtor.clone() + &type_args.print_to_string(None);
            if !expected_dtors.remove(&dtor_name) {
                return Err(Error::UnexpectedDtorInCocase {
                    span: cocase.span.to_miette(),
                    dtor: cocase.xtor.clone(),
                });
            }
            match symbol_table.dtors.get(&dtor_name) {
                None => {
                    return Err(Error::Undefined {
                        span: self.span.to_miette(),
                        name: dtor_name.clone(),
                    })
                }
                Some((dtor_args, dtor_ret_ty)) => {
                    cocase.context_names.no_dups(&dtor_name)?;
                    let context_clause = cocase.context_names.add_types(dtor_args)?;

                    let mut new_context = context.clone();
                    new_context
                        .bindings
                        .append(&mut context_clause.bindings.clone());

                    let new_rhs =
                        cocase
                            .rhs
                            .check(symbol_table, &new_context, &dtor_ret_ty.clone())?;
                    new_cocases.push(Clause {
                        rhs: new_rhs,
                        context: context_clause,
                        ..cocase
                    });
                }
            };
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

impl UsedBinders for Cocase {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.cocases.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use super::{Check, Term};
    use crate::{
        parser::fun,
        syntax::context::{Chirality::Prd, NameContext, TypingContext},
        syntax::{
            declarations::Polarity,
            terms::{Case, Clause, Lit, XVar},
            types::{Ty, TypeArgs},
        },
        test_common::symbol_table_list_template,
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_case_list() {
        let mut ctx_case_names = NameContext::default();
        ctx_case_names.bindings.push("x".to_string());
        ctx_case_names.bindings.push("xs".to_string());
        let mut ctx_case = TypingContext::default();
        ctx_case.add_var("x", Ty::mk_i64());
        ctx_case.add_var("xs", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
        let mut symbol_table = symbol_table_list_template();
        let result = Case {
            span: Span::default(),
            cases: vec![
                Clause {
                    span: Span::default(),
                    pol: Polarity::Data,
                    xtor: "Nil".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    rhs: Lit::mk(1).into(),
                },
                Clause {
                    span: Span::default(),
                    pol: Polarity::Data,
                    xtor: "Cons".to_owned(),
                    context_names: ctx_case_names.clone(),
                    context: TypingContext::default(),
                    rhs: XVar::mk("x").into(),
                },
            ],
            destructee: Rc::new(XVar::mk("x").into()),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            ty: None,
        }
        .check(&mut symbol_table, &ctx, &Ty::mk_i64())
        .unwrap();
        let expected = Case {
            span: Span::default(),
            cases: vec![
                Clause {
                    span: Span::default(),
                    pol: Polarity::Data,
                    xtor: "Nil".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    rhs: Lit::mk(1).into(),
                },
                Clause {
                    span: Span::default(),
                    pol: Polarity::Data,
                    xtor: "Cons".to_owned(),
                    context_names: ctx_case_names,
                    context: ctx_case,
                    rhs: XVar {
                        span: Span::default(),
                        var: "x".to_owned(),
                        ty: Some(Ty::mk_i64()),
                        chi: Some(Prd),
                    }
                    .into(),
                },
            ],
            destructee: Rc::new(
                XVar {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: Some(Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()]))),
                    chi: Some(Prd),
                }
                .into(),
            ),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            ty: Some(Ty::mk_i64()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_case_fail() {
        let mut ctx_names = NameContext::default();
        ctx_names.bindings.push("x".to_string());
        ctx_names.bindings.push("y".to_string());
        let mut symbol_table = symbol_table_list_template();
        let result = Case {
            span: Span::default(),
            cases: vec![Clause {
                span: Span::default(),
                pol: Polarity::Data,
                xtor: "Tup".to_owned(),
                context_names: ctx_names,
                context: TypingContext::default(),
                rhs: XVar::mk("x").into(),
            }],
            destructee: Rc::new(Lit::mk(1).into()),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            ty: None,
        }
        .check(&mut symbol_table, &TypingContext::default(), &Ty::mk_i64());
        assert!(result.is_err())
    }

    fn example_empty() -> Case {
        Case {
            span: Span::default(),
            destructee: Rc::new(XVar::mk("x").into()),
            type_args: TypeArgs::default(),
            cases: vec![],
            ty: None,
        }
    }

    fn example_tup() -> Case {
        let mut ctx_names = NameContext::default();
        ctx_names.bindings.push("x".to_string());
        ctx_names.bindings.push("y".to_string());
        Case {
            span: Span::default(),
            destructee: Rc::new(XVar::mk("x").into()),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            cases: vec![Clause {
                span: Span::default(),
                pol: Polarity::Data,
                xtor: "Tup".to_owned(),
                context_names: ctx_names,
                context: TypingContext::default(),
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
            "x.case[i64, i64] { Tup(x, y) => 2 }"
        )
    }

    #[test]
    fn parse_tup() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("x.case[i64,i64] { Tup(x,y) => 2 }"),
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
            context::{Chirality::Prd, NameContext, TypingContext},
            declarations::Polarity,
            terms::{Clause, Cocase, Lit, XVar},
            types::{Ty, TypeArgs},
        },
        test_common::{symbol_table_fun, symbol_table_lpair},
    };
    use codespan::Span;
    use printer::Print;

    #[test]
    fn check_lpair() {
        let mut symbol_table = symbol_table_lpair();
        let result = Cocase {
            span: Span::default(),
            cocases: vec![
                Clause {
                    span: Span::default(),
                    pol: Polarity::Codata,
                    xtor: "Fst".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    rhs: Lit::mk(1).into(),
                },
                Clause {
                    span: Span::default(),
                    pol: Polarity::Codata,
                    xtor: "Snd".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    rhs: Lit::mk(2).into(),
                },
            ],
            ty: None,
        }
        .check(
            &mut symbol_table,
            &TypingContext::default(),
            &Ty::mk_decl("LPair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
        )
        .unwrap();
        let expected = Cocase {
            span: Span::default(),
            cocases: vec![
                Clause {
                    span: Span::default(),
                    pol: Polarity::Codata,
                    xtor: "Fst".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    rhs: Lit::mk(1).into(),
                },
                Clause {
                    span: Span::default(),
                    pol: Polarity::Codata,
                    xtor: "Snd".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    rhs: Lit::mk(2).into(),
                },
            ],
            ty: Some(Ty::mk_decl(
                "LPair",
                TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            )),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_fun() {
        let mut ctx_names = NameContext::default();
        ctx_names.bindings.push("x".to_string());
        ctx_names.bindings.push("a".to_string());
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        ctx.add_covar("a", Ty::mk_i64());
        let mut symbol_table = symbol_table_fun();
        let result = Cocase {
            span: Span::default(),
            cocases: vec![Clause {
                span: Span::default(),
                pol: Polarity::Codata,
                xtor: "Ap".to_owned(),
                context_names: ctx_names.clone(),
                context: TypingContext::default(),
                rhs: XVar::mk("x").into(),
            }],
            ty: None,
        }
        .check(
            &mut symbol_table,
            &TypingContext::default(),
            &Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
        )
        .unwrap();
        let expected = Cocase {
            span: Span::default(),
            cocases: vec![Clause {
                span: Span::default(),
                pol: Polarity::Codata,
                xtor: "Ap".to_owned(),
                context_names: ctx_names,
                context: ctx,
                rhs: XVar {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: Some(Ty::mk_i64()),
                    chi: Some(Prd),
                }
                .into(),
            }],
            ty: Some(Ty::mk_decl(
                "Fun",
                TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            )),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_cocase_fail() {
        let mut symbol_table = symbol_table_fun();
        let result = Cocase {
            span: Span::default(),
            cocases: vec![Clause {
                span: Span::default(),
                pol: Polarity::Codata,
                xtor: "Ap".to_owned(),
                context_names: NameContext::default(),
                context: TypingContext::default(),
                rhs: Lit::mk(1).into(),
            }],
            ty: None,
        }
        .check(
            &mut symbol_table,
            &TypingContext::default(),
            &Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
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
                    pol: Polarity::Codata,
                    xtor: "Hd".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    rhs: Term::Lit(Lit::mk(2)),
                },
                Clause {
                    span: Span::default(),
                    pol: Polarity::Codata,
                    xtor: "Tl".to_owned(),
                    context_names: NameContext::default(),
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
