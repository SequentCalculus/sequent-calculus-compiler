use super::{Cns, Prd, PrdCns, Term};
use crate::{
    syntax::{Covar, Var},
    traits::{free_vars::FreeV, substitution::Subst},
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

    fn example_prodsubst() -> Vec<(Term<Prd>, Var)> {
        vec![(
            XVar {
                prdcns: Prd,
                var: "y".to_owned(),
            }
            .into(),
            "x".to_owned(),
        )]
    }

    fn example_conssubst() -> Vec<(Term<Cns>, Covar)> {
        vec![(
            XVar {
                prdcns: Cns,
                var: "b".to_owned(),
            }
            .into(),
            "a".to_owned(),
        )]
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

    #[test]
    fn subst_var1() {
        let result = XVar {
            prdcns: Prd,
            var: "x".to_owned(),
        }
        .subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XVar {
            prdcns: Prd,
            var: "y".to_owned(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_var2() {
        let result = XVar {
            prdcns: Prd,
            var: "z".to_owned(),
        }
        .subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XVar {
            prdcns: Prd,
            var: "z".to_owned(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_covar1() {
        let result = XVar {
            prdcns: Cns,
            var: "a".to_owned(),
        }
        .subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XVar {
            prdcns: Cns,
            var: "b".to_owned(),
        }
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn subst_covar2() {
        let result = XVar {
            prdcns: Cns,
            var: "c".to_owned(),
        }
        .subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XVar {
            prdcns: Cns,
            var: "c".to_owned(),
        }
        .into();
        assert_eq!(result, expected)
    }
}
