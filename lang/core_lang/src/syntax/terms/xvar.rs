//! This module defines variables and covariables in Core.

use printer::Print;

use super::{Cns, ContextBinding, FsTerm, Prd, PrdCns, Term};
use crate::{
    syntax::{Covar, FsStatement, Var, context::Chirality, types::Ty},
    traits::*,
};

use std::collections::{BTreeSet, HashSet};

/// This struct defines variables and covariables. It consists of the information that determines
/// whether it is a variable (if `T` is instantiated with [`Prd`]) or a covariable (if `T` is
/// instantiated with [`Cns`]), a name for the (co)variable, and of the type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XVar<T: PrdCns> {
    /// Whether we have a variable or covariable
    pub prdcns: T,
    /// The name of the (co)variable
    pub var: Var,
    /// The type
    pub ty: Ty,
}

impl XVar<Prd> {
    /// This function creates a variable with the given name and type.
    pub fn var(name: &str, ty: Ty) -> Self {
        XVar {
            prdcns: Prd,
            var: name.to_string(),
            ty,
        }
    }
}
impl XVar<Cns> {
    /// This function creates a covariable with the given name and type.
    pub fn covar(name: &str, ty: Ty) -> Self {
        XVar {
            prdcns: Cns,
            var: name.to_string(),
            ty,
        }
    }
}

impl<T: PrdCns> Typed for XVar<T> {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl<T: PrdCns> Print for XVar<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.var.print(cfg, alloc)
    }
}

impl<T: PrdCns> From<XVar<T>> for Term<T> {
    fn from(value: XVar<T>) -> Self {
        Term::XVar(value)
    }
}

impl<T: PrdCns> From<XVar<T>> for FsTerm<T> {
    fn from(value: XVar<T>) -> Self {
        FsTerm::XVar(value)
    }
}

impl Subst for XVar<Prd> {
    type Target = Term<Prd>;
    fn subst_sim(
        self,
        prod_subst: &[(Var, Term<Prd>)],
        _cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        match prod_subst.iter().find(|(var, _)| *var == self.var) {
            None => self.into(),
            Some((_, p)) => p.clone(),
        }
    }
}
impl Subst for XVar<Cns> {
    type Target = Term<Cns>;
    fn subst_sim(
        self,
        _prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        match cons_subst.iter().find(|(covar, _)| *covar == self.var) {
            None => self.into(),
            Some((_, p)) => p.clone(),
        }
    }
}

impl<T: PrdCns> TypedFreeVars for XVar<T> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        let chi = if self.prdcns.is_prd() {
            Chirality::Prd
        } else {
            Chirality::Cns
        };
        vars.insert(ContextBinding {
            var: self.var.clone(),
            chi,
            ty: self.ty.clone(),
        });
    }
}

impl<T: PrdCns> Bind for XVar<T> {
    fn bind(self, k: Continuation, used_var: &mut HashSet<Var>) -> FsStatement {
        let chi = if self.prdcns.is_prd() {
            Chirality::Prd
        } else {
            Chirality::Cns
        };
        let binding = ContextBinding {
            var: self.var,
            chi,
            ty: self.ty,
        };

        k(binding, used_var)
    }
}

impl<T: PrdCns> SubstVar for XVar<T> {
    type Target = XVar<T>;
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> XVar<T> {
        match subst.iter().find(|(old, _)| *old == self.var) {
            None => self,
            Some((_, new)) => {
                self.var = new.clone();
                self
            }
        }
    }
}

#[cfg(test)]
mod var_tests {

    use super::{Subst, XVar};
    use crate::{syntax::types::Ty, test_common::example_subst};

    // Substitution tests

    #[test]
    fn subst_var1() {
        let subst = example_subst();
        let result = XVar::var("x", Ty::I64).subst_sim(&subst.0, &subst.1);
        let expected = XVar::var("y", Ty::I64).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_var2() {
        let subst = example_subst();
        let result = XVar::var("z", Ty::I64).subst_sim(&subst.0, &subst.1);
        let expected = XVar::var("z", Ty::I64).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_covar1() {
        let subst = example_subst();
        let result = XVar::covar("a", Ty::I64).subst_sim(&subst.0, &subst.1);
        let expected = XVar::covar("b", Ty::I64).into();
        assert_eq!(result, expected)
    }
    #[test]
    fn subst_covar2() {
        let subst = example_subst();
        let result = XVar::covar("c", Ty::I64).subst_sim(&subst.0, &subst.1);
        let expected = XVar::covar("c", Ty::I64).into();
        assert_eq!(result, expected)
    }
}
