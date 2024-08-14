use super::{stringify_and_join, Consumer, Covar, Dtor, Producer, Var};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt};

// Destructor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Destructor {
    pub id: Dtor,
    pub producers: Vec<Producer>,
    pub consumers: Vec<Consumer>,
}

impl std::fmt::Display for Destructor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_joined: String = stringify_and_join(&self.producers);
        let coargs_joined: String = stringify_and_join(&self.consumers);
        write!(f, "{}({};{})", self.id, args_joined, coargs_joined)
    }
}

impl FreeV for Destructor {
    fn free_vars(&self) -> HashSet<Var> {
        let mut free_vars = self.producers.free_vars();
        free_vars.extend(self.consumers.free_vars());
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let mut free_covars = self.producers.free_covars();
        free_covars.extend(self.consumers.free_covars());
        free_covars
    }
}

impl From<Destructor> for Consumer {
    fn from(value: Destructor) -> Self {
        Consumer::Destructor(value)
    }
}

impl Subst for Destructor {
    type Target = Destructor;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        Destructor {
            id: self.id.clone(),
            producers: self.producers.subst_sim(prod_subst, cons_subst),
            consumers: self.consumers.subst_sim(prod_subst, cons_subst),
        }
    }
}
