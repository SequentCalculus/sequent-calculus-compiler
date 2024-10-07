use super::{Cns, Prd, PrdCns};
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

impl std::fmt::Display for XVar<Prd> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.var)
    }
}
impl std::fmt::Display for XVar<Cns> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}", self.var)
    }
}

impl FreeV for XVar<Prd> {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::from([self.var.clone()])
    }

    fn free_covars(&self) -> HashSet<Covar> {
        HashSet::new()
    }
}
impl FreeV for XVar<Cns> {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        HashSet::from([self.var.clone()])
    }
}

impl From<XVar> for Producer {
    fn from(value: XVar) -> Self {
        Producer::XVar(value)
    }
}

impl Subst for XVar {
    type Target = Producer;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        _cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        let XVar { var } = self;
        match prod_subst.iter().find(|(_, v)| v == var) {
            None => XVar { var: var.clone() }.into(),
            Some((p, _)) => p.clone(),
        }
    }
}

#[cfg(test)]
mod variable_tests {
    use std::collections::HashSet;

    use crate::{syntax::XVar, traits::free_vars::FreeV};

    #[test]
    fn display() {
        let ex = XVar {
            var: "x".to_string(),
        };
        assert_eq!(format!("{ex}"), "x")
    }

    #[test]
    fn free_vars() {
        let ex = XVar {
            var: "x".to_string(),
        };
        let mut res = HashSet::new();
        res.insert("x".to_string());
        assert_eq!(ex.free_vars(), res)
    }

    #[test]
    fn free_covars() {
        let ex = XVar {
            var: "x".to_string(),
        };
        assert_eq!(ex.free_covars(), HashSet::new())
    }
}
