//! This module defines a pattern match of a data type in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{CASE, DOT};
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::Inference;
use crate::typing::*;

use std::collections::HashMap;
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

impl Check for Case {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        // Find out the type on which we pattern match by inspecting the first clause. We throw an
        // error for empty cases.
        let (ty, expected_ctors) = match self.clauses.first() {
            Some(clause) => {
                // the name of the constructor in the symbol table for the instantiated data type
                let ctor_name = clause.xtor.clone() + &self.type_args.print_to_string(None);
                match symbol_table.lookup_ty_for_ctor(&self.span, &ctor_name) {
                    Ok(ty) => ty,
                    Err(_) => {
                        // if there is no instance yet, we create on from the template
                        symbol_table.lookup_ty_template_for_ctor(&clause.xtor, &self.type_args)?
                    }
                }
            }
            None => {
                return Err(Error::EmptyMatch { span: self.span });
            }
        };

        // We check the scrutinee `e` in `e.case {...}` against this type.
        self.scrutinee = self.scrutinee.check(symbol_table, context, &ty)?;

        let mut new_clauses = vec![];
        for ctor in expected_ctors {
            // the name of the constructor in the symbol table for the instantiated data type
            let ctor_name = ctor.clone() + &self.type_args.print_to_string(None);
            let mut clause = if let Some(position) =
                self.clauses.iter().position(|clause| clause.xtor == ctor)
            {
                self.clauses.swap_remove(position)
            } else {
                return Err(Error::MissingCtorInCase {
                    span: self.span,
                    ctor,
                });
            };
            match symbol_table.ctors.get(&ctor_name) {
                None => {
                    return Err(Error::Undefined {
                        span: Some(self.span),
                        name: ctor_name.clone(),
                    });
                }
                Some(signature) => {
                    clause.context_names.no_dups(&ctor_name)?;
                    let context_clause = clause.context_names.add_types(signature)?;

                    let mut new_context = context.clone();
                    new_context
                        .bindings
                        .append(&mut context_clause.bindings.clone());

                    clause.context = context_clause;
                    clause.body = clause.body.check(symbol_table, &new_context, expected)?;
                    new_clauses.push(clause);
                }
            }
        }

        if !self.clauses.is_empty() {
            return Err(Error::UnexpectedCtorsInCase {
                span: self.span,
                ctors: self
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

impl Inference for Case {
    fn constraint_equations(
            &mut self,
            symbol_table: &mut SymbolTable,
            context: &TypingContext,
            var_name_generator: &mut inference::VarNameGenerator,
            ty_var: Ty
        ) -> Result<Vec<(Ty,Ty)>, Error> {

        if let Some(first_clause) = self.clauses.first() {
            let mut constraints = Vec::new();

            // adding a type variable the type of the case block
            let new_type_var = var_name_generator.get_new_ty_var();
            self.ty = Some(new_type_var.clone());
            constraints.push((new_type_var, ty_var.clone()));


            let data_type_name = match symbol_table.find_xdata_type_name(&first_clause.xtor) {
                Some(type_name) => type_name,
                None => {
                    return Err(Error::Undefined {
                        span: Some(self.span),
                        name: first_clause.xtor.clone()
                    })
                }
            };

            let (chirality, general_type_vars, needed_clauses) = symbol_table.type_templates.get(&data_type_name).unwrap();

            let needed_clauses_set: HashSet<&String> = needed_clauses.iter().collect();

            if chirality == &Polarity::Codata {
                return Err(Error::ExpectedTermGotCovariable { span: self.span });
            }

            // this instance of the Codata Type is instanciated by replacing the general type vars
            // with instance type variables eg. (A -> a1)


            let mut type_var_mapping: HashMap<Name, Ty> = HashMap::new();
            
            if self.type_args.args.len() == general_type_vars.bindings.len() {
                // if the right amount of type arguments is given they are used

                for (type_var_name, given_ty) in general_type_vars.bindings.iter().zip(self.type_args.args.iter()) {
                    type_var_mapping.insert(type_var_name.clone(), given_ty.clone());
                }
            } else if self.type_args.args.is_empty() {
                // if no type Arguments are given, they are all replaced by variables,

                for type_var_name in general_type_vars.bindings.iter() {
                    type_var_mapping.insert(type_var_name.clone(), var_name_generator.get_new_ty_var());
                }
            } else {
                // if the wrong amount of type arguments are given, an error is returned

                return Err(Error::WrongNumberOfTypeArguments{
                    span: Some(self.span),
                    expected: general_type_vars.bindings.len(),
                    got: self.type_args.args.len(),
                });
            }

            let mut used_clauses = HashSet::new();

            for clause in &mut self.clauses {

                used_clauses.insert(&clause.xtor);

                // checking that that type of the clause is the same for all clauses
                match symbol_table.find_xdata_type_name(&clause.xtor) {
                    Some(type_name) => {
                        if type_name != data_type_name {
                            return Err(Error::Mismatch {
                                span: self.span,
                                expected: data_type_name,
                                got: type_name
                            });
                        }
                    },
                    None => {
                        return Err(Error::Undefined {
                            span: Some(self.span),
                            name: clause.xtor.clone()
                        });
                    }
                }

                let instantiated_arg_types = match symbol_table.ctor_templates.get(&clause.xtor) {
                    Some(arg_types) => arg_types.clone().subst_ty(&type_var_mapping),
                    None => {
                        return Err(Error::Undefined {
                            span: Some(self.span),
                            name: clause.xtor.clone()
                        })
                    }
                };

                if clause.context_names.bindings.len() != instantiated_arg_types.bindings.len() {
                    return Err(Error::WrongNumberOfBinders {
                        span: Some(self.span),
                        expected: instantiated_arg_types.bindings.len(),
                        provided: clause.context_names.bindings.len()
                    });
                }

                // The Correct Typing Context of the Clause is added, to be used later in the compiler pipeline
                clause.context = instantiated_arg_types.clone();

                // The outer Context is expanded with the bindings from the clause
                let mut clause_context = context.clone();
                for (template_arg_type, arg_name) in instantiated_arg_types.bindings.iter().zip(&clause.context_names.bindings) {
                    // if the name of a binding is already in the Context it is shadowed by the new variable
                    if let Some(index) = clause_context.bindings.iter().position(|bind| bind.var == *arg_name) {
                        clause_context.bindings.swap_remove(index);
                    }
                    clause_context.add_var(arg_name, template_arg_type.ty.clone());
                }

                // every clause must have the same out type, the expected type of the whole case block
                constraints.append(&mut clause.body.constraint_equations(
                    &mut symbol_table.clone(),
                    &clause_context,
                    var_name_generator,
                    ty_var.clone())?
                );

            }

            let unused_clauses: HashSet<&String> = needed_clauses_set.difference(&used_clauses).copied().collect();

            if !unused_clauses.is_empty() {
                return Err(Error::MissingCtorInCase { span: self.span, ctor: unused_clauses.iter().next().unwrap().to_string()});
            }


            let scutinee_type_args = TypeArgs::mk(general_type_vars.bindings.iter()
                .map(|binding| type_var_mapping.get(binding).unwrap()).cloned().collect());

            let scrutinee_type = Ty::mk_decl(&data_type_name, scutinee_type_args);

            constraints.append(&mut self.scrutinee.constraint_equations(symbol_table, context, var_name_generator,scrutinee_type)?);

            Ok(constraints)
        } else {
            // the clauses are empty, aborting the type inference
            Err(Error::Mismatch {
                span: self.span,
                expected: "At least one Clause in Case Block".to_string(),
                got: "No clause".to_string()
            })
        }
    }


    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable
    ) -> Result<(), Error> {
        for ty in &mut self.type_args.args {
            ty.mut_subst_ty(mappings);
            ty.check(&Some(self.span), symbol_table)?;
        }
        
        self.scrutinee.insert_inferred_type(mappings, symbol_table)?;

        for clause in &mut self.clauses {
            clause.body.insert_inferred_type(mappings, symbol_table)?;
            for ctx_binding in &mut clause.context.bindings {
                ctx_binding.ty.mut_subst_ty(mappings);
                ctx_binding.ty.check(&clause.context.span, symbol_table)?;
            }
        }
        
        match &mut self.ty {
            Some(ty_var) => {
                ty_var.mut_subst_ty(mappings);
                ty_var.check(&Some(self.span), symbol_table)
            },
            None => panic!("The Type of the term {:?} is not set after type inference", self)
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
    use crate::typing::inference::Inference;
    use crate::typing::inference::VarNameGenerator;
    use crate::typing::*;

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
        }
        .check(&mut symbol_table, &ctx, &Ty::mk_i64())
        .unwrap();
        let expected = Case {
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
                    context_names: ctx_case_names,
                    context: ctx_case,
                    body: XVar {
                        span: dummy_span(),
                        var: "x".to_owned(),
                        ty: Some(Ty::mk_i64()),
                        chi: Some(Prd),
                    }
                    .into(),
                },
            ],
            scrutinee: Rc::new(
                XVar {
                    span: dummy_span(),
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
        .check(&mut symbol_table, &TypingContext::default(), &Ty::mk_i64());
        assert!(result.is_err())
    }

    #[test]
    fn inference_case_list_no_annotation() {
        let mut ctx_case_names = NameContext::default();
        ctx_case_names.bindings.push("x".to_string());
        ctx_case_names.bindings.push("xs".to_string());

        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
        let mut symbol_table = symbol_table_list_template();
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

        let result = term.constraint_equations(&mut symbol_table, &ctx, &mut VarNameGenerator::new(), Ty::mk_ty_var("x")).unwrap();

        let expected = vec![
            (Ty::mk_ty_var("0"), Ty::mk_ty_var("x")),

            // Nil
            (Ty::mk_ty_var("x"), Ty::mk_i64()),

            // Cons
            (Ty::mk_ty_var("2"), Ty::mk_ty_var("x")),
            (Ty::mk_ty_var("x"), Ty::mk_ty_var("1")),

            // scrutinee
            (Ty::mk_ty_var("3"), Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("1")]))),
            (Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("1")])), Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])))
        ];
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
        let mut symbol_table = symbol_table_list_template();
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

        let result = term.constraint_equations(&mut symbol_table, &ctx, &mut VarNameGenerator::new(), Ty::mk_ty_var("x")).unwrap();

        let expected = vec![
            (Ty::mk_ty_var("0"), Ty::mk_ty_var("x")),

            // Nil
            (Ty::mk_ty_var("x"), Ty::mk_i64()),

            // Cons
            (Ty::mk_ty_var("1"), Ty::mk_ty_var("x")),
            (Ty::mk_ty_var("x"), Ty::mk_i64()),

            // scrutinee
            (Ty::mk_ty_var("2"), Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()]))),
            (Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])), Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])))
        ];
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
        let mut symbol_table = symbol_table_list_template();
        let mut term = Case {
            span: dummy_span(),
            clauses: vec![
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

        let result = term.constraint_equations(&mut symbol_table, &ctx, &mut VarNameGenerator::new(), Ty::mk_ty_var("x"));
        assert!(result.is_err_and(|e| matches!(e, Error::MissingCtorInCase { ctor, .. } if ctor == "Nil")));
    }

    #[test]
    fn inference_wrong_case() {
        let mut ctx_names = NameContext::default();
        ctx_names.bindings.push("x".to_string());
        ctx_names.bindings.push("y".to_string());
        let mut symbol_table = symbol_table_list_template();
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
        .constraint_equations(&mut symbol_table, &TypingContext::default(), &mut VarNameGenerator::new(), Ty::mk_ty_var("x"));

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
