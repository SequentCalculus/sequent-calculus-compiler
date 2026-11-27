//! This module defines the call of a top-level function in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::{
    ChoiceId, TypeInference, TypeInferenceState, TypeVarMapping, VariationalConstraint, World,
};
use crate::typing::*;

use std::collections::HashSet;

/// This struct defines the call of a top-level function in Fun. It consists of the name of the
/// top-level function to call, the arguments, and after typechecking also the inferred type.
///
/// Example:
/// `fac(10)`, calls the top-level function `fac` with argument `10`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Call {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The name of the top-level function being called
    pub name: Name,
    /// The arguments
    pub args: Arguments,
    /// The (inferred) return type
    pub ret_ty: Option<Ty>,
    /// The potential choice identifier to resolve overloading
    pub choice_id: Option<ChoiceId>,
}

impl OptTyped for Call {
    fn get_type(&self) -> Option<Ty> {
        self.ret_ty.clone()
    }
}

impl Print for Call {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.name
            .print(cfg, alloc)
            .append(self.args.print(cfg, alloc).parens().group())
    }
}

impl From<Call> for Term {
    fn from(value: Call) -> Self {
        Term::Call(value)
    }
}

impl TypeInference for Call {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        match state.symbol_table.defs.get(&self.name).cloned() {
            Some(signatures) if signatures.is_empty() => {
                panic!("encountered a function definition with no signature(s)")
            }
            Some(signatures) if signatures.len() == 1 => {
                // there is only one signature -> the function has no overloading, no need to add a variation variable
                let (types, ret_ty) = signatures[0].clone();
                self.ret_ty = Some(expected_ty.clone());

                state
                    .constraints
                    .push(VariationalConstraint::equal(expected_ty, ret_ty));

                self.args
                    .collect_constraints(&types, context, state, self.span)?;

                Ok(())
            }
            Some(signatures) => {
                // there are more than one signatures for a function -> it is overloaded

                self.ret_ty = Some(expected_ty.clone());

                // setting the choice id to resolve the overload later
                let choice = state.new_choice(signatures.len());

                self.choice_id = Some(choice);

                // creating type variables for each argument.
                // They are later used to link them and the choice to the parameter type of the overloaded function
                let mut arg_type_vars = Vec::with_capacity(self.args.entries.len());

                for _ in 0..self.args.entries.len() {
                    let new_type_var = state.fresh_names.fresh_ty_var();
                    arg_type_vars.push(new_type_var.clone());
                }

                // the constraints are created for every choice, and marked with the choice made
                for (alternative, (types, ret_ty)) in signatures.iter().enumerate() {
                    // marking impossible overloads because of the arity mismatch
                    if self.args.entries.len() != types.bindings.len() {
                        state.constraints.push(VariationalConstraint::impossible_if(
                            choice,
                            alternative,
                            Error::WrongNumberOfArguments {
                                span: self.span,
                                expected: types.bindings.len(),
                                got: self.args.entries.len(),
                            },
                        ));
                        continue;
                    }

                    // each parameter binding is now linked to the type variable of the argument together
                    // with the choice made by selecting this overload of the function
                    for ((arg, expected_type), arg_ty) in self
                        .args
                        .entries
                        .iter_mut()
                        .zip(types.bindings.iter())
                        .zip(arg_type_vars.clone())
                    {
                        // special case for covariables
                        if expected_type.chi == Cns {
                            match arg {
                                Term::XVar(variable) => {
                                    debug_assert!(
                                        variable.chi.is_some(),
                                        "The variable {} has no Chirality selected",
                                        variable.var
                                    );
                                    if variable.chi == Some(Prd) {
                                        state.constraints.push(
                                            VariationalConstraint::impossible_if(
                                                choice,
                                                alternative,
                                                Error::ExpectedCovariableGotTerm {
                                                    span: self.span,
                                                },
                                            ),
                                        );
                                    } else {
                                        let found_ty =
                                            context.lookup_covar(&variable.var, &variable.span)?;

                                        state.constraints.push(VariationalConstraint::equal_if(
                                            expected_type.ty.clone(),
                                            found_ty.clone(),
                                            choice,
                                            alternative,
                                        ));
                                    }
                                }
                                _ => {
                                    state.constraints.push(VariationalConstraint::impossible_if(
                                        choice,
                                        alternative,
                                        Error::ExpectedCovariableGotTerm { span: self.span },
                                    ));
                                }
                            }
                        } else {
                            // if the function does not expect a covar, simply link the paramter and argument type
                            // together with the choice of the function
                            state.constraints.push(VariationalConstraint::equal_if(
                                arg_ty.clone(),
                                expected_type.ty.clone(),
                                choice,
                                alternative,
                            ));
                        }
                    }

                    // the return type is linked to the choice
                    state.constraints.push(VariationalConstraint::equal_if(
                        expected_ty.clone(),
                        ret_ty.clone(),
                        choice,
                        alternative,
                    ));
                }

                // finally the constraints of the actual arguments are gather linked to their type variables
                for (arg, arg_ty) in self.args.entries.iter_mut().zip(arg_type_vars) {
                    arg.collect_constraints(state, context, arg_ty)?;
                }

                Ok(())
            }
            None => Err(Error::Undefined {
                span: Some(self.span),
                name: self.name.clone(),
            }),
        }
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        self.args
            .insert_inferred_types(type_vars, symbol_table, world)?;
        if let Some(choice) = &self.choice_id {
            let index = world.get(choice).expect(
                "Although the function call had a choice_id, the choice id was not in the world",
            );
            // insert the individual name of the function, if it is overloaded
            self.name = symbol_table::overload_name(&self.name, index);
        }

        match &mut self.ret_ty {
            Some(ty) => {
                ty.mut_subst_ty(type_vars);
                ty.check(&Some(self.span), symbol_table)
            }
            None => panic!(
                "The Type of the term {:?} is not set after type inference",
                self
            ),
        }
    }
}

impl UsedBinders for Call {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.args.entries.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use printer::Print;

    use crate::parser::fun;
    use crate::syntax::util::dummy_span;
    use crate::syntax::*;
    use crate::typing::inference::TypeInference;
    use crate::typing::inference::TypeInferenceState;
    use crate::typing::inference::VariationalConstraint;
    use crate::typing::*;

    #[test]
    fn inference_simple_def() {
        let mut symbol_table = SymbolTable::default();
        let mut typing_ctx = TypingContext::default();
        typing_ctx.add_var("x", Ty::mk_i64());
        symbol_table.defs.insert(
            "simple".to_owned(),
            vec![(typing_ctx, Ty::mk_ty_var("out_type"))],
        );

        let mut term = Call {
            span: dummy_span(),
            name: "simple".to_owned(),
            args: Arguments {
                entries: vec![Lit::mk(5).into()],
            },
            ret_ty: None,
            choice_id: None,
        };

        let mut state = TypeInferenceState {
            symbol_table,
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &TypingContext::default(), Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![
            VariationalConstraint::equal(Ty::mk_ty_var("x"), Ty::mk_ty_var("out_type")),
            VariationalConstraint::equal(Ty::mk_i64(), Ty::mk_i64()),
        ];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(result, expected);
        assert_eq!(term.ret_ty, Some(Ty::mk_ty_var("x")))
    }

    #[test]
    fn inference_missing_def() {
        let mut term = Call {
            span: dummy_span(),
            name: "simple".to_owned(),
            args: Arguments {
                entries: vec![Lit::mk(5).into()],
            },
            ret_ty: None,
            choice_id: None,
        };

        let mut state = TypeInferenceState {
            symbol_table: Default::default(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        let result =
            term.collect_constraints(&mut state, &TypingContext::default(), Ty::mk_ty_var("x"));

        assert!(
            result.is_err_and(|e| matches!(e, Error::Undefined { name, .. } if name == "simple"))
        )
    }

    fn example_simple() -> Call {
        Call {
            span: dummy_span(),
            name: "foo".to_string(),
            args: vec![].into(),
            ret_ty: None,
            choice_id: None,
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            example_simple().print_to_string(Default::default()),
            "foo()"
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("foo()"), Ok(example_simple().into()));
    }

    fn example_extended() -> Call {
        Call {
            span: dummy_span(),
            name: "foo".to_string(),
            args: vec![Term::Lit(Lit::mk(2)).into(), XVar::mk("a").into()].into(),
            ret_ty: None,
            choice_id: None,
        }
    }

    #[test]
    fn display_extended() {
        assert_eq!(
            example_extended().print_to_string(Default::default()),
            "foo(2, a)"
        )
    }

    #[test]
    fn parse_extended() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("foo(2, a)"), Ok(example_extended().into()));
    }
}
