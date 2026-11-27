//! This module defines arguments in Fun.

use miette::SourceSpan;
use printer::*;

use crate::syntax::*;
use crate::typing::{
    Error, SymbolTable,
    inference::{TypeInference, TypeInferenceState, TypeVarMapping, VariationalConstraint, World},
};

/// This struct defines arguments in Fun. They consist of a list of [`Term`]s.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Arguments {
    pub entries: Vec<Term>,
}

impl Print for Arguments {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let sep = if cfg.allow_linebreaks {
            alloc.line_()
        } else {
            alloc.nil()
        };

        if self.entries.is_empty() {
            alloc.nil()
        } else {
            sep.clone()
                .append(self.entries.print(cfg, alloc))
                .nest(cfg.indent)
                .append(sep)
        }
    }
}

impl From<Vec<Term>> for Arguments {
    fn from(bindings: Vec<Term>) -> Arguments {
        Arguments { entries: bindings }
    }
}

impl Arguments {
    /// This function collects the constraints for the arguments of a call, constructor or
    /// destructor. Producer arguments must have the type of their parameter, covariable arguments
    /// are looked up in the context and must have the type of their consumer parameter.
    /// - `params` are the parameters the arguments are passed to.
    /// - `context` is the typing context of the arguments.
    /// - `state` is the inference state.
    /// - `span` is the source location used for arity errors.
    pub fn collect_constraints(
        &mut self,
        params: &TypingContext,
        context: &TypingContext,
        state: &mut TypeInferenceState,
        span: SourceSpan,
    ) -> Result<(), Error> {
        if self.entries.len() != params.bindings.len() {
            return Err(Error::WrongNumberOfArguments {
                span,
                expected: params.bindings.len(),
                got: self.entries.len(),
            });
        }

        for (arg, param) in self.entries.iter_mut().zip(params.bindings.iter()) {
            if param.chi == Chirality::Cns {
                match arg {
                    Term::XVar(variable) => {
                        let found_ty = variable.collect_covariable_constraints(context, state)?;
                        state
                            .constraints
                            .push(VariationalConstraint::equal(param.ty.clone(), found_ty));
                    }
                    _ => return Err(Error::ExpectedCovariableGotTerm { span }),
                }
            } else {
                arg.collect_constraints(state, context, param.ty.clone())?;
            }
        }

        Ok(())
    }

    /// This function inserts the inferred types into all arguments, see
    /// [`TypeInference::insert_inferred_types`].
    pub fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        for term in &mut self.entries {
            term.insert_inferred_types(type_vars, symbol_table, world)?;
        }

        Ok(())
    }
}
