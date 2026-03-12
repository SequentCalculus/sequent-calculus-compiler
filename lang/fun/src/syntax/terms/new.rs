//! This module defines a copattern match of a codata type in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::NEW;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::Inference;
use crate::typing::*;

use std::collections::HashMap;
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

impl Check for New {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let (name, type_args) = match expected {
            Ty::I64 { .. } => {
                return Err(Error::ExpectedI64ForNew { span: self.span });
            }
            Ty::Decl {
                name, type_args, ..
            } => (name, type_args),
        };

        // the name of the instance of the data type in the symbol table, the instance must exists
        // already
        let type_name = name.clone() + &type_args.print_to_string(None);
        let expected_dtors = match symbol_table.types.get(&type_name) {
            Some((Polarity::Codata, _type_args, dtors)) => dtors.clone(),
            Some((Polarity::Data, _, _)) => {
                return Err(Error::ExpectedDataForNew {
                    span: self.span,
                    data: type_name,
                });
            }
            None => {
                return Err(Error::Undefined {
                    span: Some(self.span),
                    name: type_name,
                });
            }
        };

        let mut new_clauses = vec![];
        for dtor in expected_dtors {
            // the name of the constructor in the symbol table for the instantiated data type
            let dtor_name = dtor.clone() + &type_args.print_to_string(None);
            let mut clause = if let Some(position) =
                self.clauses.iter().position(|clause| clause.xtor == dtor)
            {
                self.clauses.swap_remove(position)
            } else {
                return Err(Error::MissingDtorInNew {
                    span: self.span,
                    dtor: dtor.clone(),
                });
            };
            match symbol_table.dtors.get(&dtor_name) {
                None => {
                    return Err(Error::Undefined {
                        span: Some(self.span),
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
            }
        }

        if !self.clauses.is_empty() {
            return Err(Error::UnexpectedDtorsInNew {
                span: self.span,
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

impl Inference for New {
    fn constraint_equations(
            &mut self,
            symbol_table: &mut SymbolTable,
            context: &TypingContext,
            var_name_generator: &mut inference::VarNameGenerator,
            ty_var: Ty
        ) -> Result<Vec<(Ty,Ty)>, Error> {

        let mut constraints: Vec<(Ty, Ty)> = Vec::new();
        
        if let Some(first_clause) = self.clauses.first() {
            let new_type_var = var_name_generator.get_new_ty_var();
            self.ty = Some(new_type_var.clone());
            constraints.push((new_type_var, ty_var.clone()));

            let data_type_name = match symbol_table.find_xdata_type_name(&first_clause.xtor) {
                Some(type_name) => type_name,
                None => {return Err(Error::Undefined { span: Some(self.span), name: first_clause.xtor.clone()})},
            };

            let (chirality, general_type_vars, _) = symbol_table.type_templates.get(&data_type_name).unwrap();

            if chirality ==&Polarity::Data {
                return Err(Error::ExpectedCovariableGotTerm { span: self.span });
            }

            // this instance of the Codata Type is instanciated by replacing the general type vars
            // with instance type variables eg. (A -> a1)

            // the mapping is created now, to ensure that the new type varibales in this new-Block stay consistent
            let mut type_var_mapping: HashMap<Name, Ty> = HashMap::new();
            for type_var in &general_type_vars.bindings {
                type_var_mapping.insert(type_var.clone(), var_name_generator.get_new_ty_var());
            }

            // Since the Codata Type has to be the same for all clauses, the type is instanciated once
            // all following clauses are checked against this type.


            // in every clause the General Type variables (A, B) are replaced by fresh type variables that are only for the current new-Block
            for clause in &mut self.clauses {
                match symbol_table.find_xdata_type_name(&clause.xtor) {
                    Some(type_name) => {
                        if type_name != data_type_name {
                            return Err(Error::Mismatch { span: self.span, expected: format!("a clause of Type {data_type_name}"), got: format!("a clause of Type {type_name}") });
                        }
                    },
                    None => {
                    return Err(Error::Undefined { span: Some(self.span.clone()), name: clause.xtor.clone() });
                    }
                };

                // the new arg types and out type are replaced
                let (arg_types, out_type) = match symbol_table.dtor_templates.get(&clause.xtor) {
                    Some((arg_types, out_type)) => {
                        let new_arg_types = arg_types.clone().subst_ty(&type_var_mapping);
                        let new_out_type = out_type.clone().subst_ty(&type_var_mapping);

                        (new_arg_types, new_out_type)
                    },
                    None => {
                        return Err(Error::Undefined {
                        span: Some(self.span),
                        name: clause.xtor.clone(),
                    })},
                };

                // the arguments are added to the context, variable that are shadowed, are replaced by the new var
                let mut clause_context = context.clone();
                for argument in arg_types.bindings {
                    if let Some(index) = clause_context.bindings.iter().position(|bind| bind.var == argument.var) {
                        clause_context.bindings.swap_remove(index);
                    }
                    clause_context.add_var(&argument.var, argument.ty);
                }

                //the argument types are now compared to the expected type of the body of the clause
                constraints.append(&mut clause.body.constraint_equations(&mut symbol_table.clone(), &clause_context, var_name_generator, out_type)?);
            
            }


            // creating the expected type for the new-block
            let mut general_arg_types = Vec::new();
            for ty_var_name in &general_type_vars.bindings {
                general_arg_types.push(
                    type_var_mapping.get(ty_var_name).unwrap().clone()
                );
            }

            let resulting_codata_type = Ty::Decl {
                span: Some(self.span),
                name: data_type_name,
                type_args: TypeArgs {
                    span: Some(self.span),
                    args: general_arg_types
                }
            };

            constraints.push((ty_var, resulting_codata_type));
            

        } else {
            // the clauses are empty, aborting the type inference
            return Err(Error::Mismatch {
                span: self.span,
                expected: "At least one Clause in New Block".to_string(),
                got: "No clause".to_string()
            });
        }
        Ok(constraints)

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
    use crate::typing::inference::Inference;
    use crate::typing::inference::VarNameGenerator;
    use crate::typing::*;

    #[test]
    fn check_lpair() {
        let mut symbol_table = symbol_table_lpair();
        let result = New {
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
        }
        .check(
            &mut symbol_table,
            &TypingContext::default(),
            &Ty::mk_decl("LPair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
        )
        .unwrap();
        let expected = New {
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
        }
        .check(
            &mut symbol_table,
            &TypingContext::default(),
            &Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
        )
        .unwrap();
        let expected = New {
            span: dummy_span(),
            clauses: vec![Clause {
                span: dummy_span(),
                pol: Polarity::Codata,
                xtor: "apply".to_owned(),
                context_names: ctx_names,
                context: ctx,
                body: XVar {
                    span: dummy_span(),
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
            span: dummy_span(),
            clauses: vec![Clause {
                span: dummy_span(),
                pol: Polarity::Codata,
                xtor: "apply".to_owned(),
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

        let mut symbol_table = symbol_table_lpair();

        let result = term.constraint_equations(&mut symbol_table, &TypingContext::default(), &mut VarNameGenerator::new(), Ty::mk_ty_var("x")).unwrap();

        let lpair_type = Some(Ty::mk_decl(
                "LPair",
                TypeArgs::mk(vec![Ty::mk_ty_var("1"), Ty::mk_ty_var("2")]),
            ));

        let expected = vec![(Ty::mk_ty_var("0"), Ty::mk_ty_var("x")), (Ty::mk_ty_var("1"), Ty::mk_i64()), (Ty::mk_ty_var("2"), Ty::mk_i64()), (Ty::mk_ty_var("x"), lpair_type.clone().unwrap())];

        assert_eq!(term.ty, Some(Ty::mk_ty_var("0")));
        assert_eq!(result, expected);
        

    }

    #[test]
    fn inference_fun() {
        let mut ctx_names = NameContext::default();
        ctx_names.bindings.push("x".to_string());
        ctx_names.bindings.push("a".to_string());
        let mut symbol_table = symbol_table_fun();
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

        let result = term.constraint_equations(&mut symbol_table, &TypingContext::default(), &mut VarNameGenerator::new(), Ty::mk_ty_var("x")).unwrap();

        let expected_codata_type = Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_ty_var("1"), Ty::mk_ty_var("2")]));

        let expected = vec![
            (Ty::mk_ty_var("0"), Ty::mk_ty_var("x")),
            (Ty::mk_ty_var("3"), Ty::mk_ty_var("2")),
            (Ty::mk_ty_var("2"), Ty::mk_ty_var("1")),
            (Ty::mk_ty_var("x"), expected_codata_type)];

        assert_eq!(term.ty, Some(Ty::mk_ty_var("0")));
        assert_eq!(result, expected);

        
    }

    #[test]
    /// This Test tests a Codata Type with it's own Template as a Type
    fn inference_stream() {
        let mut symbol_table = symbol_table_stream();
        let mut term = New {
            span: dummy_span(),
            clauses: vec![
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Codata,
                    xtor: "head".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Lit::mk(1).into()
                },
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Codata,
                    xtor: "tail".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: XVar::mk("y").into()
                }
            ],
            ty: None
        };
        let mut ctx = TypingContext::default();
        // to make it a smaller test, the recursive call of the Stream is put into the variable y with the Stream[y], hence the type argument is already instantiated
        ctx.add_var("y", Ty::mk_decl("Stream", TypeArgs::mk(vec![Ty::mk_ty_var("y")])));

        let result = term.constraint_equations(&mut symbol_table, &ctx, &mut VarNameGenerator::new(), Ty::mk_ty_var("x")).unwrap();

        let expected_codata_type = Ty::mk_decl("Stream", TypeArgs::mk(vec![Ty::mk_ty_var("1")]));

        let expected = vec![
            (Ty::mk_ty_var("0"), Ty::mk_ty_var("x")),
            (Ty::mk_ty_var("1"), Ty::mk_i64()),
            (Ty::mk_ty_var("2"), expected_codata_type.clone()),
            (expected_codata_type.clone(), Ty::mk_decl("Stream", TypeArgs::mk(vec![Ty::mk_ty_var("y")]))),
            (Ty::mk_ty_var("x"), expected_codata_type)
        ];

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("0")))

        
    }

    #[test]
    /// this test checks whether a data Clause in a New-Block is detected as an error
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

        let result = term.constraint_equations(&mut symbol_table, &TypingContext::default(), &mut VarNameGenerator::new(), Ty::mk_ty_var("x"));
        assert!(result.is_err_and(|f| matches!(f, Error::Mismatch{..})))

    }

    #[test]
    /// this test checks that an unknown xtor returns an "undefined" error
    fn inference_unknown_clause() {

        // unknown type in an empty symbol table
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
            ],
            ty: None,
        };

        let result = term.constraint_equations(&mut SymbolTable::default(), &TypingContext::default(), &mut VarNameGenerator::new(), Ty::mk_ty_var("x"));

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
