use codespan::Span;
use derivative::Derivative;
use printer::{theme::ThemeExt, tokens::COCASE, DocAllocator, Print};

use super::{print_clauses, Clause, Term};
use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        declarations::Polarity,
        types::{OptTyped, Ty},
        used_binders::UsedBinders,
        Var,
    },
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use std::collections::HashSet;

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
        mut self,
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
        let expected_dtors = match symbol_table.types.get(&type_name) {
            Some((Polarity::Codata, _type_args, dtors)) => dtors.clone(),
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
        for dtor in expected_dtors {
            let dtor_name = dtor.clone() + &type_args.print_to_string(None);
            let mut cocase = if let Some(position) =
                self.cocases.iter().position(|cocase| cocase.xtor == dtor)
            {
                self.cocases.swap_remove(position)
            } else {
                return Err(Error::MissingDtorInCocase {
                    span: self.span.to_miette(),
                    dtor: dtor.clone(),
                });
            };
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

                    cocase.context = context_clause;
                    cocase.rhs =
                        cocase
                            .rhs
                            .check(symbol_table, &new_context, &dtor_ret_ty.clone())?;
                    new_cocases.push(cocase);
                }
            };
        }

        if !self.cocases.is_empty() {
            return Err(Error::UnexpectedDtorsInCocase {
                span: self.span.to_miette(),
                dtors: self
                    .cocases
                    .iter()
                    .map(|cocase| cocase.xtor.clone())
                    .collect::<Vec<_>>()
                    .print_to_string(None),
            });
        }
        self.cocases = new_cocases;

        self.ty = Some(expected.clone());
        Ok(self)
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
                xtor: "Apply".to_owned(),
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
                xtor: "Apply".to_owned(),
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
                xtor: "Apply".to_owned(),
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
