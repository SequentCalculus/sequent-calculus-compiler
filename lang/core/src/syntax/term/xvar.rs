use super::{Cns, Prd, PrdCns, Term};
use crate::{
    syntax::{Covar, Var},
    traits::{free_vars::FreeV, substitution::Subst},
};
use std::{collections::HashSet, fmt};

/// Either a variable or a covariable:
/// - A variable if `T = Prd`
/// - A covariable if `T = Cns`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XVar<T: PrdCns> {
    pub prdcns: T,
    pub var: Var,
}

impl XVar<Prd> {
    /// Create a new variable with the given name.
    pub fn var(name: &str) -> Self {
        XVar {
            prdcns: Prd,
            var: name.to_string(),
        }
    }
}

impl XVar<Cns> {
    /// Create a new covariable with the given name.
    pub fn covar(name: &str) -> Self {
        XVar {
            prdcns: Cns,
            var: name.to_string(),
        }
    }
}

impl<T: PrdCns> std::fmt::Display for XVar<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = if self.prdcns.is_prd() { "" } else { "'" };
        write!(f, "{}{}", prefix, self.var)
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

impl<T: PrdCns> From<XVar<T>> for Term<T> {
    fn from(value: XVar<T>) -> Self {
        Term::XVar(value)
    }
}
impl Subst for XVar<Prd> {
    type Target = Term<Prd>;

    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        _cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        let XVar { prdcns: _, var } = self;
        match prod_subst.iter().find(|(_, v)| v == var) {
            None => XVar {
                prdcns: Prd,
                var: var.clone(),
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
        let XVar { prdcns: _, var } = self;
        match cons_subst.iter().find(|(_, cv)| cv == var) {
            None => XVar {
                prdcns: Cns,
                var: var.clone(),
            }
            .into(),
            Some((p, _)) => p.clone(),
        }
    }
}

#[cfg(test)]
mod var_tests {
    use super::{FreeV, Subst, Term, XVar};
    use crate::syntax::{
        term::{Cns, Prd},
        Covar, Var,
    };
    use std::collections::HashSet;

    // Display tests

    #[test]
    fn display_var() {
        let result = format!("{}", XVar::var("x"));
        let expected = "x";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_covar() {
        let result = format!("{}", XVar::covar("a"));
        let expected = "'a";
        assert_eq!(result, expected)
    }

    // Free variable tests

    #[test]
    fn free_vars_var() {
        let result = XVar::var("x").free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_covar() {
        let result = XVar::covar("a").free_vars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_var() {
        let result = XVar::var("x").free_covars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_covar() {
        let result = XVar::covar("a").free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    // Substitution tests

    fn example_prodsubst() -> Vec<(Term<Prd>, Var)> {
        vec![(XVar::var("y").into(), "x".to_owned())]
    }

    fn example_conssubst() -> Vec<(Term<Cns>, Covar)> {
        vec![(XVar::covar("b").into(), "a".to_owned())]
    }

    #[test]
    fn subst_var1() {
        let result = XVar::var("x").subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XVar::var("y").into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_var2() {
        let result = XVar::var("z").subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XVar::var("z").into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_covar1() {
        let result = XVar::covar("a").subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XVar::covar("b").into();
        assert_eq!(result, expected)
    }
    #[test]
    fn subst_covar2() {
        let result = XVar::covar("c").subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XVar::covar("c").into();
        assert_eq!(result, expected)
    }
}
