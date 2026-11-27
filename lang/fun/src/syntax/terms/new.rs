//! This module defines a copattern match of a codata type in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::NEW;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::TypeInference;
use crate::typing::inference::TypeInferenceState;
use crate::typing::inference::VariationalConstraint;
use crate::typing::inference::{TypeVarMapping, World};
use crate::typing::*;

use std::collections::HashSet;

/// This struct defines a copattern match of a codata type. It consists of a list of clauses, and
/// after typechecking also of the inferred type.
///
/// Example:
/// ```text
/// new { Head => 1, Tail => const1() }
/// ```
/// constructs a stream with head `1` and tail `const1()`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct New {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The list of clauses
    pub clauses: Vec<Clause>,
    /// The (inferred) type of the term
    pub ty: Option<Ty>,
}

impl OptTyped for New {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for New {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
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

impl TypeInference for New {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        let Some(first_clause) = self.clauses.first() else {
            return Err(Error::EmptyMatch { span: self.span });
        };
        self.ty = Some(expected_ty.clone());

        // the codata type implemented is the one declaring the destructor of the first clause
        let (codata_name, type_params, declared_xtors) = state
            .symbol_table
            .codata_type_of_dtor(&first_clause.xtor, self.span)?;

        // the type parameters of the codata type are instantiated with fresh type variables once,
        // so they stay consistent across all clauses of this new-block
        let instantiation = type_params.instantiate(&[], self.span, &mut state.fresh_names)?;

        for clause in &mut self.clauses {
            // checking that that type of the clause is the same for all clauses
            let (clause_type, _, _) = state
                .symbol_table
                .codata_type_of_dtor(&clause.xtor, self.span)?;
            if clause_type != codata_name {
                return Err(Error::Mismatch {
                    span: self.span,
                    expected: format!("a clause of Type {codata_name}"),
                    got: format!("a clause of Type {clause_type}"),
                });
            }

            let (dtor_params, out_type) = match state.symbol_table.dtor_templates.get(&clause.xtor)
            {
                Some((params, out_type)) => (
                    params.clone().subst_ty(&instantiation),
                    out_type.clone().subst_ty(&instantiation),
                ),
                None => {
                    return Err(Error::Undefined {
                        span: Some(self.span),
                        name: clause.xtor.clone(),
                    });
                }
            };

            // the body of a clause must have the return type of its destructor
            clause.collect_constraints(&dtor_params, context, state, out_type)?;
        }

        check_and_order_clauses(
            &mut self.clauses,
            Polarity::Codata,
            &declared_xtors,
            self.span,
        )?;

        // creating the expected type for the new-block
        let codata_type =
            Ty::mk_instance(&codata_name, &type_params, &instantiation, Some(self.span));
        state
            .constraints
            .push(VariationalConstraint::equal(expected_ty, codata_type));
        Ok(())
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        for clause in &mut self.clauses {
            clause.insert_inferred_types(type_vars, symbol_table, world)?;
        }

        Ty::insert_inferred_annotation(&mut self.ty, self.span, type_vars, symbol_table)
    }
}

impl UsedBinders for New {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.clauses.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use printer::Print;

    use crate::parser::fun;
    use crate::syntax::util::dummy_span;
    use crate::syntax::*;
    use crate::test_common::*;
    use crate::typing::inference::TypeInference;
    use crate::typing::inference::TypeInferenceState;
    use crate::typing::inference::VariationalConstraint;
    use crate::typing::*;

    #[test]
    fn inference_lpair() {
        let mut term = New {
            span: dummy_span(),
            clauses: vec![
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Codata,
                    xtor: "fst".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Lit::mk(1).into(),
                },
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Codata,
                    xtor: "snd".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Lit::mk(2).into(),
                },
            ],
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table: symbol_table_lpair(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &TypingContext::default(), Ty::mk_ty_var("x"))
            .unwrap();

        let lpair_type = Some(Ty::mk_decl(
            "LPair",
            TypeArgs::mk(vec![Ty::mk_ty_var("0"), Ty::mk_ty_var("1")]),
        ));

        let expected = vec![
            VariationalConstraint::equal(Ty::mk_ty_var("0"), Ty::mk_i64()),
            VariationalConstraint::equal(Ty::mk_ty_var("1"), Ty::mk_i64()),
            VariationalConstraint::equal(Ty::mk_ty_var("x"), lpair_type.clone().unwrap()),
        ];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(term.ty, Some(Ty::mk_ty_var("x")));
        assert_eq!(result, expected);
    }

    #[test]
    fn inference_fun() {
        let mut ctx_names = NameContext::default();
        ctx_names.bindings.push("x".to_string());
        ctx_names.bindings.push("a".to_string());
        let mut term = New {
            span: dummy_span(),
            clauses: vec![Clause {
                span: dummy_span(),
                pol: Polarity::Codata,
                xtor: "apply".to_owned(),
                context_names: ctx_names.clone(),
                context: TypingContext::default(),
                body: XVar::mk("x").into(),
            }],
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table: symbol_table_fun(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &TypingContext::default(), Ty::mk_ty_var("x"))
            .unwrap();

        let expected_codata_type = Ty::mk_decl(
            "Fun",
            TypeArgs::mk(vec![Ty::mk_ty_var("0"), Ty::mk_ty_var("1")]),
        );

        let expected = vec![
            VariationalConstraint::equal(Ty::mk_ty_var("1"), Ty::mk_ty_var("0")),
            VariationalConstraint::equal(Ty::mk_ty_var("x"), expected_codata_type),
        ];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(term.ty, Some(Ty::mk_ty_var("x")));
        assert_eq!(result, expected);
    }

    #[test]
    /// This Test tests a Codata Type with it's own Template as a Type
    fn inference_stream() {
        let mut term = New {
            span: dummy_span(),
            clauses: vec![
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Codata,
                    xtor: "head".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Lit::mk(1).into(),
                },
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Codata,
                    xtor: "tail".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: XVar::mk("y").into(),
                },
            ],
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table: symbol_table_stream(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        let mut ctx = TypingContext::default();
        // to make it a smaller test, the recursive call of the Stream is put into the variable y with the Stream[y], hence the type argument is already instantiated
        ctx.add_var(
            "y",
            Ty::mk_decl("Stream", TypeArgs::mk(vec![Ty::mk_ty_var("y")])),
        );

        term.collect_constraints(&mut state, &ctx, Ty::mk_ty_var("x"))
            .unwrap();

        let expected_codata_type = Ty::mk_decl("Stream", TypeArgs::mk(vec![Ty::mk_ty_var("0")]));

        let expected = vec![
            VariationalConstraint::equal(Ty::mk_ty_var("0"), Ty::mk_i64()),
            VariationalConstraint::equal(
                expected_codata_type.clone(),
                Ty::mk_decl("Stream", TypeArgs::mk(vec![Ty::mk_ty_var("y")])),
            ),
            VariationalConstraint::equal(Ty::mk_ty_var("x"), expected_codata_type),
        ];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("x")))
    }

    #[test]
    /// this test checks whether a data Clause in a New-Block is detected as an error. A
    /// constructor is no destructor, so it is undefined as a clause of a New-Block.
    fn inference_data_clause() {
        let mut symbol_table = symbol_table_lpair();
        symbol_table.combine(symbol_table_list());

        let mut term = New {
            span: dummy_span(),
            clauses: vec![
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Codata,
                    xtor: "fst".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Lit::mk(1).into(),
                },
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Codata,
                    xtor: "Nil".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Lit::mk(2).into(),
                },
            ],
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table: symbol_table,
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        let result =
            term.collect_constraints(&mut state, &TypingContext::default(), Ty::mk_ty_var("x"));
        assert!(result.is_err_and(|f| matches!(f, Error::Undefined { .. })))
    }

    #[test]
    /// testing, that a missing clause causes an error
    fn inference_missing_clause() {
        let mut state = TypeInferenceState {
            symbol_table: symbol_table_lpair(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        let result = New {
            span: dummy_span(),
            clauses: vec![Clause {
                span: dummy_span(),
                pol: Polarity::Codata,
                xtor: "fst".to_owned(),
                context_names: NameContext::default(),
                context: TypingContext::default(),
                body: Lit::mk(1).into(),
            }],
            ty: None,
        }
        .collect_constraints(&mut state, &TypingContext::default(), Ty::mk_ty_var("x"));

        assert!(
            result
                .is_err_and(|e| matches!(e, Error::MissingDtorInNew { dtor, .. } if dtor == "snd"))
        );
    }

    #[test]
    /// this test checks that an unknown xtor returns an "undefined" error
    fn inference_unknown_clause() {
        // unknown type in an empty symbol table
        let mut term = New {
            span: dummy_span(),
            clauses: vec![Clause {
                span: dummy_span(),
                pol: Polarity::Codata,
                xtor: "fst".to_owned(),
                context_names: NameContext::default(),
                context: TypingContext::default(),
                body: Lit::mk(1).into(),
            }],
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table: Default::default(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        let result =
            term.collect_constraints(&mut state, &TypingContext::default(), Ty::mk_ty_var("x"));

        assert!(result.is_err_and(|f| matches!(f, Error::Undefined { .. })));
    }

    fn example_empty() -> New {
        New {
            span: dummy_span(),
            clauses: vec![],
            ty: None,
        }
    }

    fn example_stream() -> New {
        New {
            span: dummy_span(),
            clauses: vec![
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Codata,
                    xtor: "head".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Term::Lit(Lit::mk(2)),
                },
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Codata,
                    xtor: "tail".to_owned(),
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
            "new {\n    head => 2,\n    tail => 4\n}"
        )
    }

    #[test]
    fn parse_stream() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("new { head => 2, tail => 4 }"),
            Ok(example_stream().into())
        );
    }
}
