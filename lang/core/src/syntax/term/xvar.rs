use printer::{tokens::TICK, DocAllocator, Print};

use super::{Cns, FsTerm, Prd, PrdCns, Term};
use crate::{
    syntax::Chirality,
    syntax::{
        types::{Ty, Typed},
        Covar, Var,
    },
    traits::{
        focus::{Bind, Continuation, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::{Subst, SubstVar},
    },
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

impl<T: PrdCns> Focusing for XVar<T> {
    type Target = crate::syntax::term::xvar::FsXVar;
    fn focus(self, state: &mut FocusingState) -> Self::Target {
        let chi = if (self.prdcns.is_prd() && !self.ty.is_codata(state.codata_types))
            || (self.prdcns.is_cns() && self.ty.is_codata(state.codata_types))
        {
            crate::syntax::Chirality::Prd
        } else {
            crate::syntax::Chirality::Cns
        };
        crate::syntax::term::xvar::FsXVar { chi, var: self.var }
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

/// Either a variable or a covariable:
/// - A variable if `T = Prd`
/// - A covariable if `T = Cns`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsXVar {
    pub chi: Chirality,
    pub var: Var,
}

impl FsXVar {
    /// Create a new variable with the given name.
    #[must_use]
    pub fn var(name: &str) -> Self {
        FsXVar {
            chi: Chirality::Prd,
            var: name.to_string(),
        }
    }
    #[must_use]
    pub fn covar(name: &str) -> Self {
        FsXVar {
            chi: Chirality::Cns,
            var: name.to_string(),
        }
    }
}

impl Print for FsXVar {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(&self.var)
    }
}

impl From<FsXVar> for FsTerm {
    fn from(value: FsXVar) -> Self {
        FsTerm::XVar(value)
    }
}

impl SubstVar for FsXVar {
    type Target = FsXVar;

    fn subst_sim(mut self, subst: &[(Var, Var)]) -> FsXVar {
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
    use printer::Print;

    use super::{FreeV, Subst, Term, XVar};
    use crate::syntax::{
        term::{Cns, Prd},
        types::Ty,
        Covar, Var,
    };
    use std::collections::HashSet;

    // Display tests

    #[test]
    fn display_var() {
        let result = XVar::var("x", Ty::Int()).print_to_string(None);
        let expected = "x";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_covar() {
        let result = XVar::covar("a", Ty::Int()).print_to_string(None);
        let expected = "'a";
        assert_eq!(result, expected)
    }

    // Free variable tests

    #[test]
    fn free_vars_var() {
        let result = XVar::var("x", Ty::Int()).free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_covar() {
        assert!(XVar::covar("a", Ty::Int()).free_vars().is_empty())
    }

    #[test]
    fn free_covars_var() {
        assert!(XVar::var("x", Ty::Int()).free_covars().is_empty())
    }

    #[test]
    fn free_covars_covar() {
        let result = XVar::covar("a", Ty::Int()).free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    // Substitution tests

    fn example_prodsubst() -> Vec<(Term<Prd>, Var)> {
        vec![(XVar::var("y", Ty::Int()).into(), "x".to_owned())]
    }

    fn example_conssubst() -> Vec<(Term<Cns>, Covar)> {
        vec![(XVar::covar("b", Ty::Int()).into(), "a".to_owned())]
    }

    #[test]
    fn subst_var1() {
        let result =
            XVar::var("x", Ty::Int()).subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XVar::var("y", Ty::Int()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_var2() {
        let result =
            XVar::var("z", Ty::Int()).subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XVar::var("z", Ty::Int()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_covar1() {
        let result =
            XVar::covar("a", Ty::Int()).subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XVar::covar("b", Ty::Int()).into();
        assert_eq!(result, expected)
    }
    #[test]
    fn subst_covar2() {
        let result =
            XVar::covar("c", Ty::Int()).subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XVar::covar("c", Ty::Int()).into();
        assert_eq!(result, expected)
    }
}
