use super::{stringify_and_join, Clause, Consumer, Covar, Dtor, Producer, Var};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt};

// Cocase
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cocase {
    pub cocases: Vec<Clause<Dtor>>,
}

impl std::fmt::Display for Cocase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clauses_joined: String = stringify_and_join(&self.cocases);
        write!(f, "cocase {{ {} }}", clauses_joined)
    }
}

impl FreeV for Cocase {
    fn free_vars(&self) -> HashSet<Var> {
        self.cocases.free_vars()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.cocases.free_covars()
    }
}

impl From<Cocase> for Producer {
    fn from(value: Cocase) -> Self {
        Producer::Cocase(value)
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

#[cfg(test)]
mod cocase_test {
    use crate::syntax::Cocase;

    #[test]
    fn display() {
        let ex = Cocase { cocases: vec![] };
        assert_eq!(format!("{ex}"), "cocase {  }".to_string());
    }
}
