//! This module defines a pattern match of a data type in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{CASE, DOT};
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::{Constraint, ConstraintBank, Inference};
use crate::typing::*;

use std::collections::HashMap;
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

impl Inference for Case {
    fn gather_constraints(
        &mut self,
        constraint_bank: &mut ConstraintBank,
        context: &TypingContext,
        ty_var: Ty,
    ) -> Result<(), Error> {
        if let Some(first_clause) = self.clauses.first() {
            // adding a type variable the type of the case block
            let new_type_var = constraint_bank.var_name_generator.get_new_ty_var();
            self.ty = Some(new_type_var.clone());
            constraint_bank
                .constraints
                .push(Constraint::mk_only_ty(new_type_var, ty_var.clone()));

            let data_type_name = match constraint_bank
                .symbol_table
                .find_xdata_type_name(&first_clause.xtor)
            {
                Some(type_name) => type_name,
                None => {
                    return Err(Error::Undefined {
                        span: Some(self.span),
                        name: first_clause.xtor.clone(),
                    });
                }
            };

            let (chirality, general_type_vars, needed_clauses) = constraint_bank
                .symbol_table
                .type_templates
                .get(&data_type_name)
                .unwrap()
                .clone();

            if chirality == Polarity::Codata {
                return Err(Error::ExpectedTermGotCovariable { span: self.span });
            }

            // this instance of the Codata Type is instanciated by replacing the general type vars
            // with instance type variables eg. (A -> a1)

            let mut type_var_mapping: HashMap<Name, Ty> = HashMap::new();

            if self.type_args.args.len() == general_type_vars.bindings.len() {
                // if the right amount of type arguments is given they are used

                for (type_var_name, given_ty) in general_type_vars
                    .bindings
                    .iter()
                    .zip(self.type_args.args.iter())
                {
                    type_var_mapping.insert(type_var_name.clone(), given_ty.clone());
                }
            } else if self.type_args.args.is_empty() {
                // if no type Arguments are given, they are all replaced by variables,

                for type_var_name in general_type_vars.bindings.iter() {
                    type_var_mapping.insert(
                        type_var_name.clone(),
                        constraint_bank.var_name_generator.get_new_ty_var(),
                    );
                }
            } else {
                // if the wrong amount of type arguments are given, an error is returned

                return Err(Error::WrongNumberOfTypeArguments {
                    span: Some(self.span),
                    expected: general_type_vars.bindings.len(),
                    got: self.type_args.args.len(),
                });
            }

            let mut used_clauses = Vec::new();

            for clause in &mut self.clauses {
                used_clauses.push(&clause.xtor);

                // checking that that type of the clause is the same for all clauses
                match constraint_bank
                    .symbol_table
                    .find_xdata_type_name(&clause.xtor)
                {
                    Some(type_name) => {
                        if type_name != data_type_name {
                            return Err(Error::Mismatch {
                                span: self.span,
                                expected: data_type_name,
                                got: type_name,
                            });
                        }
                    }
                    None => {
                        return Err(Error::Undefined {
                            span: Some(self.span),
                            name: clause.xtor.clone(),
                        });
                    }
                }

                let mut instantiated_arg_types = match constraint_bank
                    .symbol_table
                    .ctor_templates
                    .get(&clause.xtor)
                {
                    Some(arg_types) => arg_types.clone().subst_ty(&type_var_mapping),
                    None => {
                        return Err(Error::Undefined {
                            span: Some(self.span),
                            name: clause.xtor.clone(),
                        });
                    }
                };

                if clause.context_names.bindings.len() != instantiated_arg_types.bindings.len() {
                    return Err(Error::WrongNumberOfBinders {
                        span: Some(self.span),
                        expected: instantiated_arg_types.bindings.len(),
                        provided: clause.context_names.bindings.len(),
                    });
                }

                // renaming the arguments to fit with the name bindings from the clause
                for (new_name, arg_binding) in zip(
                    clause.context_names.bindings.iter(),
                    instantiated_arg_types.bindings.iter_mut(),
                ) {
                    arg_binding.var = new_name.clone();
                }

                // The correct typing context of the clause is added, to be used later in the compiler pipeline
                clause.context = instantiated_arg_types.clone();

                // The outer context is expanded with the bindings from the clause
                let mut full_clause_context = context.clone();
                for arg_binding in instantiated_arg_types.bindings {
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
                clause.body.gather_constraints(
                    constraint_bank,
                    &full_clause_context,
                    ty_var.clone(),
                )?;
            }

            // check that all clauses were also used
            for clause_name in needed_clauses.iter() {
                if !used_clauses.contains(&clause_name) {
                    return Err(Error::MissingCtorInCase {
                        span: self.span,
                        ctor: clause_name.clone(),
                    });
                }
            }

            // later stages in the pipeline expect the clauses in the same order as in the Data definition
            self.clauses.sort_by_key(|c| {
                needed_clauses
                    .iter()
                    .position(|name| &c.xtor == name)
                    .unwrap_or(usize::MAX)
            });

            let scutinee_type_args = TypeArgs::mk(
                general_type_vars
                    .bindings
                    .iter()
                    .map(|binding| type_var_mapping.get(binding).unwrap())
                    .cloned()
                    .collect(),
            );

            let scrutinee_type = Ty::mk_decl(&data_type_name, scutinee_type_args);

            self.scrutinee
                .gather_constraints(constraint_bank, context, scrutinee_type)?;

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

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<u32, usize>,
    ) -> Result<(), Error> {
        for ty in &mut self.type_args.args {
            ty.mut_subst_ty(mappings);
            ty.check(&Some(self.span), symbol_table)?;
        }

        self.scrutinee
            .insert_inferred_type(mappings, symbol_table, choices)?;

        for clause in &mut self.clauses {
            clause
                .body
                .insert_inferred_type(mappings, symbol_table, choices)?;
            for ctx_binding in &mut clause.context.bindings {
                ctx_binding.ty.mut_subst_ty(mappings);
                ctx_binding.ty.check(&clause.context.span, symbol_table)?;
            }
        }

        match &mut self.ty {
            Some(ty_var) => {
                ty_var.mut_subst_ty(mappings);
                ty_var.check(&Some(self.span), symbol_table)
            }
            None => panic!(
                "The Type of the term {:?} is not set after type inference",
                self
            ),
        }
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
    use crate::typing::inference::Constraint;
    use crate::typing::inference::ConstraintBank;
    use crate::typing::inference::Inference;
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

        let mut constraint_bank = ConstraintBank {
            symbol_table,
            var_name_generator: Default::default(),
            constraints: Default::default(),
            possible_choices: Default::default(),
        };

        term.gather_constraints(&mut constraint_bank, &ctx, Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![
            Constraint::mk_only_ty(Ty::mk_ty_var("0"), Ty::mk_ty_var("x")),
            // Nil
            Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_i64()),
            // Cons
            Constraint::mk_only_ty(Ty::mk_ty_var("2"), Ty::mk_ty_var("x")),
            Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_ty_var("1")),
            // scrutinee
            Constraint::mk_only_ty(
                Ty::mk_ty_var("3"),
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("1")])),
            ),
            Constraint::mk_only_ty(
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("1")])),
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
            ),
        ];

        let ConstraintBank {
            constraints: result,
            ..
        } = constraint_bank;

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("0")));
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

        let mut constraint_bank = ConstraintBank {
            symbol_table,
            var_name_generator: Default::default(),
            constraints: Default::default(),
            possible_choices: Default::default(),
        };

        term.gather_constraints(&mut constraint_bank, &ctx, Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![
            Constraint::mk_only_ty(Ty::mk_ty_var("0"), Ty::mk_ty_var("x")),
            // Nil
            Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_i64()),
            // Cons
            Constraint::mk_only_ty(Ty::mk_ty_var("1"), Ty::mk_ty_var("x")),
            Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_i64()),
            // scrutinee
            Constraint::mk_only_ty(
                Ty::mk_ty_var("2"),
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
            ),
            Constraint::mk_only_ty(
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
            ),
        ];

        let ConstraintBank {
            constraints: result,
            ..
        } = constraint_bank;

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("0")));
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

        let mut constraint_bank = ConstraintBank {
            symbol_table,
            var_name_generator: Default::default(),
            constraints: Default::default(),
            possible_choices: Default::default(),
        };

        let result = term.gather_constraints(&mut constraint_bank, &ctx, Ty::mk_ty_var("x"));
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

        let mut constraint_bank = ConstraintBank {
            symbol_table,
            var_name_generator: Default::default(),
            constraints: Default::default(),
            possible_choices: Default::default(),
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
        .gather_constraints(
            &mut constraint_bank,
            &TypingContext::default(),
            Ty::mk_ty_var("x"),
        );

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
