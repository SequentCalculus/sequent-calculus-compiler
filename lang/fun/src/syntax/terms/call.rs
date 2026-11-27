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
        let Some(signatures) = state.symbol_table.defs.get(&self.name).cloned() else {
            return Err(Error::Undefined {
                span: Some(self.span),
                name: self.name.clone(),
            });
        };
        self.ret_ty = Some(expected_ty.clone());

        match signatures.as_slice() {
            [] => panic!("encountered a function definition with no signature(s)"),
            [(params, ret_ty)] => {
                self.collect_call_constraints(params, ret_ty, state, context, expected_ty)
            }
            _ => self.collect_overloaded_call_constraints(&signatures, state, context, expected_ty),
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

        Ty::insert_inferred_annotation(&mut self.ret_ty, self.span, type_vars, symbol_table)
    }
}

impl Call {
    /// This function collects the constraints for the call of a function that is not overloaded.
    /// - `params` are the parameters of the function.
    /// - `ret_ty` is the return type of the function.
    /// - `state` is the inference state.
    /// - `context` is the typing context of the call.
    /// - `expected_ty` is the type the call must have.
    fn collect_call_constraints(
        &mut self,
        params: &TypingContext,
        ret_ty: &Ty,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        state
            .constraints
            .push(VariationalConstraint::equal(expected_ty, ret_ty.clone()));

        self.args
            .collect_constraints(params, context, state, self.span)
    }

    /// This function collects the constraints for the call of an overloaded function. The call
    /// introduces a choice between the overloads, and the constraints linking the arguments and
    /// the return type to the signature of an overload hold only if this overload is selected.
    /// - `signatures` are the parameters and return types of all overloads.
    /// - `state` is the inference state.
    /// - `context` is the typing context of the call.
    /// - `expected_ty` is the type the call must have.
    fn collect_overloaded_call_constraints(
        &mut self,
        signatures: &[(TypingContext, Ty)],
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        // setting the choice id to resolve the overload later
        let choice = state.new_choice(signatures.len());
        self.choice_id = Some(choice);

        // Whether an argument is a covariable does not depend on the overload, but only on how it
        // is bound in the context. A covariable argument has the type of the covariable. For a
        // producer argument, a fresh type variable stands for its type, so that the argument is
        // collected only once and only the link to the parameter types depends on the choice.
        let mut arg_types = Vec::with_capacity(self.args.entries.len());
        for arg in &mut self.args.entries {
            match arg {
                Term::XVar(variable) if is_covariable(variable, context) => arg_types.push((
                    Cns,
                    variable.collect_covariable_constraints(context, state)?,
                )),
                _ => arg_types.push((Prd, state.fresh_names.fresh_ty_var())),
            }
        }

        // the constraints are created for every alternative, and marked with the alternative made
        for (alternative, (params, ret_ty)) in signatures.iter().enumerate() {
            // marking impossible overloads because of the arity mismatch
            if arg_types.len() != params.bindings.len() {
                state.constraints.push(VariationalConstraint::impossible_if(
                    choice,
                    alternative,
                    Error::WrongNumberOfArguments {
                        span: self.span,
                        expected: params.bindings.len(),
                        got: arg_types.len(),
                    },
                ));
                continue;
            }

            // each parameter is linked to its argument under the alternative selecting this
            // overload, an argument of the wrong chirality rules out this overload
            for ((arg_chi, arg_ty), param) in arg_types.iter().zip(params.bindings.iter()) {
                let constraint = match (arg_chi, &param.chi) {
                    (Prd, Prd) => VariationalConstraint::equal_if(
                        arg_ty.clone(),
                        param.ty.clone(),
                        choice,
                        alternative,
                    ),
                    (Cns, Cns) => VariationalConstraint::equal_if(
                        param.ty.clone(),
                        arg_ty.clone(),
                        choice,
                        alternative,
                    ),
                    (Cns, Prd) => VariationalConstraint::impossible_if(
                        choice,
                        alternative,
                        Error::ExpectedTermGotCovariable { span: self.span },
                    ),
                    (Prd, Cns) => VariationalConstraint::impossible_if(
                        choice,
                        alternative,
                        Error::ExpectedCovariableGotTerm { span: self.span },
                    ),
                };
                state.constraints.push(constraint);
            }

            // the return type is linked to the alternative
            state.constraints.push(VariationalConstraint::equal_if(
                expected_ty.clone(),
                ret_ty.clone(),
                choice,
                alternative,
            ));
        }

        // finally the producer arguments are collected against their type variables
        for (arg, (arg_chi, arg_ty)) in self.args.entries.iter_mut().zip(arg_types) {
            if arg_chi == Prd {
                arg.collect_constraints(state, context, arg_ty)?;
            }
        }

        Ok(())
    }
}

/// This function decides whether a (co)variable passed as argument is a covariable, i.e., whether
/// it is bound as a covariable in the context.
fn is_covariable(variable: &XVar, context: &TypingContext) -> bool {
    match &variable.chi {
        Some(chi) => *chi == Cns,
        None => context.lookup_covar(&variable.var, &variable.span).is_ok(),
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
