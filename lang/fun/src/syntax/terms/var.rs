//! This module defines variables and covariables in Fun.

use std::collections::HashMap;

use derivative::Derivative;
use miette::SourceSpan;
use printer::*;

use crate::syntax::*;
use crate::traits::OptTyped;
use crate::typing::inference::{Constraint, ConstraintBank, Inference};
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

impl Inference for XVar {
    fn gather_constraints(
        &mut self,
        constraint_bank: &mut ConstraintBank,
        context: &TypingContext,
        ty_var: Ty,
    ) -> Result<(), Error> {
        // Free covariables must only occur in special positions (`goto` and `arguments`)
        // and are thus rejected in all other positions by the `check` function for `XVar`.
        if self.chi == Some(Cns) {
            return Err(Error::ExpectedTermGotCovariable { span: self.span });
        }

        let found_ty = context.lookup_var(&self.var, &self.span)?;

        self.ty = Some(ty_var.clone());
        self.chi = Some(Prd);

        constraint_bank
            .constraints
            .push(Constraint::mk_only_ty(ty_var, found_ty));
        Ok(())
    }

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        _choices: &HashMap<u32, usize>,
    ) -> Result<(), Error> {
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

#[cfg(test)]
mod test {
    use crate::syntax::*;
    use crate::typing::inference::{Constraint, ConstraintBank, Inference};

    #[test]
    fn inference_var() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());

        let mut constraint_bank = ConstraintBank {
            symbol_table: Default::default(),
            var_name_generator: Default::default(),
            constraints: Default::default(),
            possible_choices: Default::default(),
        };

        let mut term = XVar::mk("x");

        term.gather_constraints(&mut constraint_bank, &ctx, Ty::mk_ty_var("x"))
            .unwrap();

        let ConstraintBank {
            constraints: result,
            ..
        } = constraint_bank;

        assert_eq!(term.ty, Some(Ty::mk_ty_var("x")));
        assert_eq!(
            result,
            vec![Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_i64())]
        )
    }
}
