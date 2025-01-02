use printer::{tokens::TICK, DocAllocator, Print};

use super::{Cns, FsTerm, Prd, PrdCns, Term};
use crate::{
    syntax::{types::Ty, Covar, Var},
    traits::*,
};

use std::collections::HashSet;

/// Either a variable or a covariable:
/// - A variable if `T = Prd`
/// - A covariable if `T = Cns`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XVar<T: PrdCns> {
    pub prdcns: T,
    pub var: Var,
    pub ty: Ty,
}

impl XVar<Prd> {
    /// Create a new variable with the given name.
    #[must_use]
    pub fn var(name: &str, ty: Ty) -> Self {
        XVar {
            prdcns: Prd,
            var: name.to_string(),
            ty,
        }
    }
}
impl XVar<Cns> {
    /// Create a new covariable with the given name.
    #[must_use]
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
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.prdcns.is_prd() {
            alloc.text(&self.var)
        } else {
            alloc.text(TICK).append(alloc.text(&self.var))
        }
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

impl<T: PrdCns> FreeV for XVar<T> {
    fn free_vars(&self) -> HashSet<Var> {
        if self.prdcns.is_prd() {
            HashSet::from([self.var.clone()])
        } else {
            HashSet::new()
        }
    }

    fn free_covars(&self) -> HashSet<Covar> {
        if self.prdcns.is_cns() {
            HashSet::from([self.var.clone()])
        } else {
            HashSet::new()
        }
    }
}

impl Subst for XVar<Prd> {
    type Target = Term<Prd>;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        _cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        match prod_subst.iter().find(|(_, var)| var == &self.var) {
            None => XVar {
                prdcns: Prd,
                var: self.var.clone(),
                ty: self.ty.clone(),
            }
            .into(),
            Some((p, _)) => p.clone(),
        }
    }
}
impl Subst for XVar<Cns> {
    type Target = Term<Cns>;
    fn subst_sim(
        &self,
        _prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        match cons_subst.iter().find(|(_, covar)| covar == &self.var) {
            None => XVar {
                prdcns: Cns,
                var: self.var.clone(),
                ty: self.ty.clone(),
            }
            .into(),
            Some((p, _)) => p.clone(),
        }
    }
}

impl<T: PrdCns> Bind for XVar<T> {
    fn bind(
        self,
        k: Continuation,
        state: &mut FocusingState,
    ) -> crate::syntax::statement::FsStatement {
        k(self.var, state)
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

    use super::{FreeV, Subst, XVar};
    use crate::{syntax::types::Ty, test_common::example_subst};
    use std::collections::HashSet;

    // Free variable tests

    #[test]
    fn free_vars_var() {
        assert_eq!(
            XVar::var("x", Ty::I64).free_vars(),
            HashSet::from(["x".to_string()])
        )
    }

    #[test]
    fn free_vars_covar() {
        assert!(XVar::covar("a", Ty::I64).free_vars().is_empty())
    }

    #[test]
    fn free_covars_var() {
        assert!(XVar::var("x", Ty::I64).free_covars().is_empty())
    }

    #[test]
    fn free_covars_covar() {
        assert_eq!(
            XVar::covar("a", Ty::I64).free_covars(),
            HashSet::from(["a".to_string()])
        )
    }

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
