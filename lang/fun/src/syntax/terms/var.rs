//! This module defines variables and covariables in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::*;

use crate::syntax::*;
use crate::traits::OptTyped;
use crate::typing::inference::{TypeInference, TypeInferenceState, VariationalConstraint};
use crate::typing::inference::{TypeVarMapping, World};
use crate::typing::*;

/// This struct defines variables and covariables. It consists of the name of the (co)variable, and
/// after typechecking also of the chirality which determines whether this is a variable or
/// covariable and the inferred type.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct XVar {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The name of the (co)variable
    pub var: Var,
    /// The (inferred) type
    pub ty: Option<Ty>,
    /// The chirality, i.e, whether this is a variable or covariable
    pub chi: Option<Chirality>,
}

impl XVar {
    /// This function returns a (co)variable from a given string, without chirality and type
    /// information.
    pub fn mk(var: &str) -> Self {
        use crate::syntax::util::dummy_span;

        XVar {
            span: dummy_span(),
            var: var.to_string(),
            ty: None,
            chi: None,
        }
    }
}

impl OptTyped for XVar {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for XVar {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.var.print(cfg, alloc)
    }
}

impl From<XVar> for Term {
    fn from(value: XVar) -> Self {
        Term::XVar(value)
    }
}

impl TypeInference for XVar {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        // Free covariables must only occur in special positions (`goto` and `arguments`)
        // and are thus rejected in all other positions by the `check` function for `XVar`.
        if self.chi == Some(Cns) {
            return Err(Error::ExpectedTermGotCovariable { span: self.span });
        }

        let found_ty = context.lookup_var(&self.var, &self.span)?;

        self.ty = Some(expected_ty.clone());
        self.chi = Some(Prd);

        state
            .constraints
            .push(VariationalConstraint::equal(expected_ty, found_ty));
        Ok(())
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        _world: &World,
    ) -> Result<(), Error> {
        match &mut self.ty {
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

#[cfg(test)]
mod test {
    use crate::syntax::*;
    use crate::typing::inference::{TypeInference, TypeInferenceState, VariationalConstraint};

    #[test]
    fn inference_var() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());

        let mut state = TypeInferenceState {
            symbol_table: Default::default(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        let mut term = XVar::mk("x");

        term.collect_constraints(&mut state, &ctx, Ty::mk_ty_var("x"))
            .unwrap();

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(term.ty, Some(Ty::mk_ty_var("x")));
        assert_eq!(
            result,
            vec![VariationalConstraint::equal(
                Ty::mk_ty_var("x"),
                Ty::mk_i64()
            )]
        )
    }
}
