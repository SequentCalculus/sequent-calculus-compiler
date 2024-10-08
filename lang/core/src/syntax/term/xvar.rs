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
