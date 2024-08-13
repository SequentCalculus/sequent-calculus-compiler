use super::{Consumer, Covar, Ctor, Producer, Var};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt};

// Constructor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor {
    pub id: Ctor,
    pub producers: Vec<Producer>,
    pub consumers: Vec<Consumer>,
}

impl std::fmt::Display for Constructor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_joined: String = self
            .producers
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let coargs_joined: String = self
            .consumers
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}({};{})", self.id, args_joined, coargs_joined)
    }
}

impl FreeV for Constructor {
    fn free_vars(&self) -> HashSet<Var> {
        let free_args = self.producers.free_vars();
        let free_coargs = self.consumers.free_vars();
        free_args.union(&free_coargs).cloned().collect()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let free_args = self.producers.free_covars();
        let free_coargs = self.consumers.free_covars();
        free_args.union(&free_coargs).cloned().collect()
    }
}

impl From<Constructor> for Producer {
    fn from(value: Constructor) -> Self {
        Producer::Constructor(value)
    }
}

impl Subst for Constructor {
    type Target = Constructor;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        Constructor {
            id: self.id.clone(),
            producers: self.producers.subst_sim(prod_subst, cons_subst),
            consumers: self.consumers.subst_sim(prod_subst, cons_subst),
        }
    }
}

#[cfg(test)]
mod constructor_tests {

    use crate::syntax::Ctor;

    use super::Constructor;

    #[test]
    fn display() {
        let ex = Constructor {
            id: Ctor::Cons,
            producers: vec![],
            consumers: vec![],
        };
        assert_eq!(format!("{ex}"), "Cons(;)".to_string())
    }
}
