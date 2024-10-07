use super::{PrdCns, Term};
use crate::{
    syntax::{Covar, Var},
    traits::{free_vars::FreeV, substitution::Subst},
};
use std::{collections::HashSet, fmt};

// Literal
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal<T: PrdCns> {
    pub prdcns: T,
    pub lit: i64,
}

impl<T: PrdCns> std::fmt::Display for Literal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lit)
    }
}

impl<T: PrdCns> FreeV for Literal<T> {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        HashSet::new()
    }
}

impl<T: PrdCns> From<Literal<T>> for Term<T> {
    fn from(value: Literal<T>) -> Self {
        Term::Literal(value)
    }
}

impl Subst for Literal {
    type Target = Literal;

    fn subst_sim(
        &self,
        _prod_subst: &[(Producer, Var)],
        _cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        self.clone()
    }
}
