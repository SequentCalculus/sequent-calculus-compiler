use super::{PrdCns, Term};
use crate::{
    syntax::{Covar, Var},
    traits::free_vars::FreeV,
};
use std::{collections::HashSet, fmt};

// XVar
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XVar<T: PrdCns> {
    pub prdcns: T,
    pub var: Var,
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

#[cfg(test)]
mod var_tests {
    use super::{FreeV, XVar};
    use crate::syntax::term::{Cns, Prd};
    use std::collections::HashSet;

    fn example_var() -> XVar<Prd> {
        XVar {
            prdcns: Prd,
            var: "x".to_owned(),
        }
    }

    fn example_covar() -> XVar<Cns> {
        XVar {
            prdcns: Cns,
            var: "a".to_owned(),
        }
    }

    #[test]
    fn display_var() {
        let result = format!("{}", example_var());
        let expected = "x";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_covar() {
        let result = format!("{}", example_covar());
        let expected = "'a";
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_var() {
        let result = example_var().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_covar() {
        let result = example_covar().free_vars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_var() {
        let result = example_var().free_covars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_covar() {
        let result = example_covar().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }
}
