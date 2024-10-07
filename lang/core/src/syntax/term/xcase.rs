use super::{PrdCns, Term};
use crate::{
    syntax::{stringify_and_join, Clause, Covar, Var},
    traits::{free_vars::FreeV, substitution::Subst},
};
use std::{collections::HashSet, fmt};

// Cocase
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XCase<T: PrdCns> {
    pub prdcns: T,
    pub clauses: Vec<Clause>,
}
impl<T: PrdCns> std::fmt::Display for XCase<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clauses_joined: String = stringify_and_join(&self.clauses);
        let prefix = if self.prdcns.is_prd() {
            "cocase"
        } else {
            "case"
        };
        write!(f, "{} {{ {} }}", prefix, clauses_joined)
    }
}

impl<T: PrdCns> FreeV for XCase<T> {
    fn free_vars(&self) -> HashSet<Var> {
        self.clauses.free_vars()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.clauses.free_covars()
    }
}

impl<T: PrdCns> From<XCase<T>> for Term<T> {
    fn from(value: XCase<T>) -> Self {
        Term::XCase(value)
    }
}

impl Subst for Cocase {
    type Target = Cocase;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        Cocase {
            cocases: self.cocases.subst_sim(prod_subst, cons_subst),
        }
    }
}
