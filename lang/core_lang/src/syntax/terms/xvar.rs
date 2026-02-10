//! This module defines variables and covariables in Core.

use printer::*;

use crate::syntax::*;
use crate::traits::*;

use std::collections::{BTreeSet, HashSet};

/// This struct defines variables and covariables. It consists of the information that determines
/// whether it is a variable (if `C` is instantiated with [`Prd`]) or a covariable (if `C` is
/// instantiated with [`Cns`]), a name for the (co)variable, and of the type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XVar<C: Chi> {
    /// Whether we have a variable or covariable
    pub prdcns: C,
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

impl<C: Chi> Typed for XVar<C> {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl<C: Chi> Print for XVar<C> {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.var.print(cfg, alloc)
    }
}

impl<C: Chi> From<XVar<C>> for Term<C> {
    fn from(value: XVar<C>) -> Self {
        Term::XVar(value)
    }
}

impl<C: Chi> From<XVar<C>> for FsTerm<C> {
    fn from(value: XVar<C>) -> Self {
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

impl<C: Chi> TypedFreeVars for XVar<C> {
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

impl<C: Chi> Bind for XVar<C> {
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

impl<C: Chi> SubstVar for XVar<C> {
    type Target = XVar<C>;
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> XVar<C> {
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
    use super::Subst;
    use crate::test_common::example_subst;
    extern crate self as core_lang;
    use core_macros::{covar, var};

    // Substitution tests

    #[test]
    fn subst_var1() {
        let subst = example_subst();
        let result = var!("x").subst_sim(&subst.0, &subst.1);
        let expected = var!("y").into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_var2() {
        let subst = example_subst();
        let result = var!("z").subst_sim(&subst.0, &subst.1);
        let expected = var!("z").into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_covar1() {
        let subst = example_subst();
        let result = covar!("a").subst_sim(&subst.0, &subst.1);
        let expected = covar!("b").into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_covar2() {
        let subst = example_subst();
        let result = covar!("c").subst_sim(&subst.0, &subst.1);
        let expected = covar!("c").into();
        assert_eq!(result, expected)
    }
}
