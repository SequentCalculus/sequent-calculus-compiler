//! This module defines a pattern match of a data type in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{CASE, DOT};
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::{TypeInference, TypeInferenceState};
use crate::typing::inference::{TypeVarMapping, World};
use crate::typing::*;

use std::iter::zip;
use std::{collections::HashSet, rc::Rc};

/// This struct defines a pattern match of a data type. It consists of the scrutinee on which to
/// match, a list of type arguments instantiating the type parameters of the data type, a list of
/// clauses, and after typechecking also of the inferred type.
///
/// Example:
/// ```text
/// l.case[i64] { Nil => 0, Cons(x, xs) => 1 + len(xs) }
/// ```
/// matches on list `l` with type argument `i64`, i.e., requires the list to be `List[i64]`. It
/// has clauses for the patterns `Nil` and `Cons(x, xs)`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Case {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The term to be matched on
    pub scrutinee: Rc<Term>,
    /// The type arguments instantiating the type parameters of the type
    pub type_args: TypeArgs,
    /// The list of clauses
    pub clauses: Vec<Clause>,
    /// The (inferred) type of the term
    pub ty: Option<Ty>,
}

impl OptTyped for Case {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Case {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        if matches!(*self.scrutinee, Term::Destructor(_)) {
            self.scrutinee
                .print(cfg, alloc)
                .append(alloc.line_())
                .append(DOT)
                .append(alloc.keyword(CASE))
                .append(self.type_args.print(cfg, alloc))
                .append(alloc.space())
                .append(print_clauses(&self.clauses, cfg, alloc))
                .nest(cfg.indent)
                .align()
        } else {
            self.scrutinee
                .print(cfg, alloc)
                .append(DOT)
                .append(alloc.keyword(CASE))
                .append(self.type_args.print(cfg, alloc))
                .append(alloc.space())
                .append(print_clauses(&self.clauses, cfg, alloc))
        }
    }
}

impl From<Case> for Term {
    fn from(value: Case) -> Self {
        Term::Case(value)
    }
}

impl TypeInference for Case {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        if let Some(first_clause) = self.clauses.first() {
            self.ty = Some(expected_ty.clone());

            // the data type matched on is the one declaring the constructor of the first clause
            let (data_name, type_params, declared_xtors) = state
                .symbol_table
                .data_type_of_ctor(&first_clause.xtor, self.span)?;

            // the type parameters of the data type are instantiated with the annotated type
            // arguments or with fresh type variables
            let instantiation =
                type_params.instantiate(&self.type_args.args, self.span, &mut state.fresh_names)?;

            let mut seen_xtors = Vec::new();

            for clause in &mut self.clauses {
                seen_xtors.push(&clause.xtor);

                // checking that that type of the clause is the same for all clauses
                let (clause_type, _, _) = state
                    .symbol_table
                    .data_type_of_ctor(&clause.xtor, self.span)?;
                if clause_type != data_name {
                    return Err(Error::Mismatch {
                        span: self.span,
                        expected: data_name,
                        got: clause_type,
                    });
                }

                let mut clause_context = match state.symbol_table.ctor_templates.get(&clause.xtor) {
                    Some(arg_types) => arg_types.clone().subst_ty(&instantiation),
                    None => {
                        return Err(Error::Undefined {
                            span: Some(self.span),
                            name: clause.xtor.clone(),
                        });
                    }
                };

                if clause.context_names.bindings.len() != clause_context.bindings.len() {
                    return Err(Error::WrongNumberOfBinders {
                        span: Some(self.span),
                        expected: clause_context.bindings.len(),
                        provided: clause.context_names.bindings.len(),
                    });
                }

                // renaming the arguments to fit with the name bindings from the clause
                for (new_name, arg_binding) in zip(
                    clause.context_names.bindings.iter(),
                    clause_context.bindings.iter_mut(),
                ) {
                    arg_binding.var = new_name.clone();
                }

                // The correct typing context of the clause is added, to be used later in the compiler pipeline
                clause.context = clause_context.clone();

                // The outer context is expanded with the bindings from the clause
                let mut full_clause_context = context.clone();
                for arg_binding in clause_context.bindings {
                    // if the name of a binding is already in the Context it is shadowed by the new variable
                    if let Some(index) = full_clause_context
                        .bindings
                        .iter()
                        .position(|bind| bind.var == *arg_binding.var)
                    {
                        full_clause_context.bindings[index] = arg_binding;
                    } else {
                        full_clause_context.bindings.push(arg_binding);
                    }
                }

                // every clause must have the same out type, the expected type of the whole case block
                clause.body.collect_constraints(
                    state,
                    &full_clause_context,
                    expected_ty.clone(),
                )?;
            }

            // check that all clauses were also used
            for clause_name in declared_xtors.iter() {
                if !seen_xtors.contains(&clause_name) {
                    return Err(Error::MissingCtorInCase {
                        span: self.span,
                        ctor: clause_name.clone(),
                    });
                }
            }

            // later stages in the pipeline expect the clauses in the same order as in the Data definition
            self.clauses.sort_by_key(|c| {
                declared_xtors
                    .iter()
                    .position(|name| &c.xtor == name)
                    .unwrap_or(usize::MAX)
            });

            let scrutinee_type = Ty::mk_instance(&data_name, &type_params, &instantiation, None);

            self.scrutinee
                .collect_constraints(state, context, scrutinee_type)?;

            Ok(())
        } else {
            // the clauses are empty, aborting the type inference
            Err(Error::Mismatch {
                span: self.span,
                expected: "At least one Clause in Case Block".to_string(),
                got: "No clause".to_string(),
            })
        }
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        for ty in &mut self.type_args.args {
            ty.insert_inferred(&Some(self.span), type_vars, symbol_table)?;
        }

        self.scrutinee
            .insert_inferred_types(type_vars, symbol_table, world)?;

        for clause in &mut self.clauses {
            clause
                .body
                .insert_inferred_types(type_vars, symbol_table, world)?;
            for ctx_binding in &mut clause.context.bindings {
                ctx_binding
                    .ty
                    .insert_inferred(&clause.context.span, type_vars, symbol_table)?;
            }
        }

        Ty::insert_inferred_annotation(&mut self.ty, self.span, type_vars, symbol_table)
    }
}

impl UsedBinders for Case {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.scrutinee.used_binders(used);
        self.clauses.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use printer::*;

    use crate::parser::fun;
    use crate::syntax::util::dummy_span;
    use crate::syntax::*;
    use crate::test_common::*;
    use crate::typing::inference::TypeInference;
    use crate::typing::inference::TypeInferenceState;
    use crate::typing::inference::VariationalConstraint;
    use crate::typing::*;

    use std::rc::Rc;

    #[test]
    fn inference_case_list_no_annotation() {
        let mut ctx_case_names = NameContext::default();
        ctx_case_names.bindings.push("x".to_string());
        ctx_case_names.bindings.push("xs".to_string());

        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
        let symbol_table = symbol_table_list_template();
        let mut term = Case {
            span: dummy_span(),
            clauses: vec![
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Data,
                    xtor: "Nil".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Lit::mk(1).into(),
                },
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Data,
                    xtor: "Cons".to_owned(),
                    context_names: ctx_case_names.clone(),
                    context: TypingContext::default(),
                    body: XVar::mk("x").into(),
                },
            ],
            scrutinee: Rc::new(XVar::mk("x").into()),
            type_args: TypeArgs::mk(vec![]),
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table,
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &ctx, Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![
            // Nil
            VariationalConstraint::equal(Ty::mk_ty_var("x"), Ty::mk_i64()),
            // Cons
            VariationalConstraint::equal(Ty::mk_ty_var("x"), Ty::mk_ty_var("0")),
            // scrutinee
            VariationalConstraint::equal(
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("0")])),
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
            ),
        ];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("x")));
    }

    #[test]
    fn inference_case_list_with_annotation() {
        let mut ctx_case_names = NameContext::default();
        ctx_case_names.bindings.push("x".to_string());
        ctx_case_names.bindings.push("xs".to_string());

        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
        let symbol_table = symbol_table_list_template();
        let mut term = Case {
            span: dummy_span(),
            clauses: vec![
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Data,
                    xtor: "Nil".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Lit::mk(1).into(),
                },
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Data,
                    xtor: "Cons".to_owned(),
                    context_names: ctx_case_names.clone(),
                    context: TypingContext::default(),
                    body: XVar::mk("x").into(),
                },
            ],
            scrutinee: Rc::new(XVar::mk("x").into()),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table,
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &ctx, Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![
            // Nil
            VariationalConstraint::equal(Ty::mk_ty_var("x"), Ty::mk_i64()),
            // Cons
            VariationalConstraint::equal(Ty::mk_ty_var("x"), Ty::mk_i64()),
            // scrutinee,
            VariationalConstraint::equal(
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
            ),
        ];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("x")));
    }

    #[test]
    fn inference_not_all_cases() {
        let mut ctx_case_names = NameContext::default();
        ctx_case_names.bindings.push("x".to_string());
        ctx_case_names.bindings.push("xs".to_string());
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
        let symbol_table = symbol_table_list_template();
        let mut term = Case {
            span: dummy_span(),
            clauses: vec![Clause {
                span: dummy_span(),
                pol: Polarity::Data,
                xtor: "Cons".to_owned(),
                context_names: ctx_case_names.clone(),
                context: TypingContext::default(),
                body: XVar::mk("x").into(),
            }],
            scrutinee: Rc::new(XVar::mk("x").into()),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table,
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        let result = term.collect_constraints(&mut state, &ctx, Ty::mk_ty_var("x"));
        assert!(
            result.is_err_and(
                |e| matches!(e, Error::MissingCtorInCase { ctor, .. } if ctor == "Nil")
            )
        );
    }

    #[test]
    fn inference_wrong_case() {
        let mut ctx_names = NameContext::default();
        ctx_names.bindings.push("x".to_string());
        ctx_names.bindings.push("y".to_string());
        let symbol_table = symbol_table_list_template();

        let mut state = TypeInferenceState {
            symbol_table,
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        let result = Case {
            span: dummy_span(),
            clauses: vec![Clause {
                span: dummy_span(),
                pol: Polarity::Data,
                xtor: "Tup".to_owned(),
                context_names: ctx_names,
                context: TypingContext::default(),
                body: XVar::mk("x").into(),
            }],
            scrutinee: Rc::new(Lit::mk(1).into()),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            ty: None,
        }
        .collect_constraints(&mut state, &TypingContext::default(), Ty::mk_ty_var("x"));

        assert!(result.is_err())
    }

    fn example_empty() -> Case {
        Case {
            span: dummy_span(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            type_args: TypeArgs::default(),
            clauses: vec![],
            ty: None,
        }
    }

    fn example_tup() -> Case {
        let mut ctx_names = NameContext::default();
        ctx_names.bindings.push("x".to_string());
        ctx_names.bindings.push("y".to_string());
        Case {
            span: dummy_span(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            clauses: vec![Clause {
                span: dummy_span(),
                pol: Polarity::Data,
                xtor: "Tup".to_owned(),
                context_names: ctx_names,
                context: TypingContext::default(),
                body: Term::Lit(Lit::mk(2)),
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
