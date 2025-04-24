use codespan::Span;
use derivative::Derivative;
use printer::{DocAllocator, Print, theme::ThemeExt, tokens::NEW};

use super::{Clause, Term, print_clauses};
use crate::{
    parser::util::ToMiette,
    syntax::{
        Var,
        context::TypingContext,
        declarations::Polarity,
        types::{OptTyped, Ty},
    },
    traits::used_binders::UsedBinders,
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use std::collections::HashSet;

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct New {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub clauses: Vec<Clause>,
    pub ty: Option<Ty>,
}

impl OptTyped for New {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for New {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(NEW)
            .append(alloc.space())
            .append(print_clauses(&self.clauses, cfg, alloc))
    }
}

impl From<New> for Term {
    fn from(value: New) -> Self {
        Term::New(value)
    }
}

impl Check for New {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let (name, type_args) = match expected {
            Ty::I64 { .. } => {
                return Err(Error::ExpectedI64ForNew {
                    span: self.span.to_miette(),
                });
            }
            Ty::Decl {
                name, type_args, ..
            } => (name, type_args),
        };

        let type_name = name.clone() + &type_args.print_to_string(None);
        let expected_dtors = match symbol_table.types.get(&type_name) {
            Some((Polarity::Codata, _type_args, dtors)) => dtors.clone(),
            Some((Polarity::Data, _, _)) => {
                return Err(Error::ExpectedDataForNew {
                    span: self.span.to_miette(),
                    data: type_name,
                });
            }
            None => {
                return Err(Error::Undefined {
                    span: self.span.to_miette(),
                    name: type_name,
                });
            }
        };

        let mut new_clauses = vec![];
        for dtor in expected_dtors {
            let dtor_name = dtor.clone() + &type_args.print_to_string(None);
            let mut clause = if let Some(position) =
                self.clauses.iter().position(|clause| clause.xtor == dtor)
            {
                self.clauses.swap_remove(position)
            } else {
                return Err(Error::MissingDtorInNew {
                    span: self.span.to_miette(),
                    dtor: dtor.clone(),
                });
            };
            match symbol_table.dtors.get(&dtor_name) {
                None => {
                    return Err(Error::Undefined {
                        span: self.span.to_miette(),
                        name: dtor_name.clone(),
                    });
                }
                Some((dtor_args, dtor_ret_ty)) => {
                    clause.context_names.no_dups(&dtor_name)?;
                    let context_clause = clause.context_names.add_types(dtor_args)?;

                    let mut new_context = context.clone();
                    new_context
                        .bindings
                        .append(&mut context_clause.bindings.clone());

                    clause.context = context_clause;
                    clause.body =
                        clause
                            .body
                            .check(symbol_table, &new_context, &dtor_ret_ty.clone())?;
                    new_clauses.push(clause);
                }
            };
        }

        if !self.clauses.is_empty() {
            return Err(Error::UnexpectedDtorsInNew {
                span: self.span.to_miette(),
                dtors: self
                    .clauses
                    .iter()
                    .map(|clause| clause.xtor.clone())
                    .collect::<Vec<_>>()
                    .print_to_string(None),
            });
        }
        self.clauses = new_clauses;

        self.ty = Some(expected.clone());
        Ok(self)
    }
}

impl UsedBinders for New {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.clauses.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use super::{Check, Term};
    use crate::{
        parser::fun,
        syntax::{
            context::{Chirality::Prd, NameContext, TypingContext},
            declarations::Polarity,
            terms::{Clause, Lit, New, XVar},
            types::{Ty, TypeArgs},
        },
        test_common::{symbol_table_fun, symbol_table_lpair},
    };
    use codespan::Span;
    use printer::Print;

    #[test]
    fn check_lpair() {
        let mut symbol_table = symbol_table_lpair();
        let result = New {
            span: Span::default(),
            clauses: vec![
                Clause {
                    span: Span::default(),
                    pol: Polarity::Codata,
                    xtor: "Fst".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Lit::mk(1).into(),
                },
                Clause {
                    span: Span::default(),
                    pol: Polarity::Codata,
                    xtor: "Snd".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Lit::mk(2).into(),
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
        let expected = New {
            span: Span::default(),
            clauses: vec![
                Clause {
                    span: Span::default(),
                    pol: Polarity::Codata,
                    xtor: "Fst".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Lit::mk(1).into(),
                },
                Clause {
                    span: Span::default(),
                    pol: Polarity::Codata,
                    xtor: "Snd".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Lit::mk(2).into(),
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
        let result = New {
            span: Span::default(),
            clauses: vec![Clause {
                span: Span::default(),
                pol: Polarity::Codata,
                xtor: "Apply".to_owned(),
                context_names: ctx_names.clone(),
                context: TypingContext::default(),
                body: XVar::mk("x").into(),
            }],
            ty: None,
        }
        .check(
            &mut symbol_table,
            &TypingContext::default(),
            &Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
        )
        .unwrap();
        let expected = New {
            span: Span::default(),
            clauses: vec![Clause {
                span: Span::default(),
                pol: Polarity::Codata,
                xtor: "Apply".to_owned(),
                context_names: ctx_names,
                context: ctx,
                body: XVar {
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
    fn check_new_fail() {
        let mut symbol_table = symbol_table_fun();
        let result = New {
            span: Span::default(),
            clauses: vec![Clause {
                span: Span::default(),
                pol: Polarity::Codata,
                xtor: "Apply".to_owned(),
                context_names: NameContext::default(),
                context: TypingContext::default(),
                body: Lit::mk(1).into(),
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

    fn example_empty() -> New {
        New {
            span: Span::default(),
            clauses: vec![],
            ty: None,
        }
    }

    fn example_stream() -> New {
        New {
            span: Span::default(),
            clauses: vec![
                Clause {
                    span: Span::default(),
                    pol: Polarity::Codata,
                    xtor: "Hd".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Term::Lit(Lit::mk(2)),
                },
                Clause {
                    span: Span::default(),
                    pol: Polarity::Codata,
                    xtor: "Tl".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Term::Lit(Lit::mk(4)),
                },
            ],
            ty: None,
        }
    }

    #[test]
    fn display_empty() {
        assert_eq!(
            example_empty().print_to_string(Default::default()),
            "new { }"
        )
    }

    #[test]
    fn parse_empty() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("new { }"), Ok(example_empty().into()));
    }

    #[test]
    fn display_stream() {
        assert_eq!(
            example_stream().print_to_string(Default::default()),
            "new {\n    Hd => 2,\n    Tl => 4\n}"
        )
    }

    #[test]
    fn parse_stream() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("new { Hd => 2, Tl => 4 }"),
            Ok(example_stream().into())
        );
    }
}
