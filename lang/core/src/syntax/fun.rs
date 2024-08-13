use super::{Consumer, Covar, Name, Producer, Statement, Var};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt};

// Fun
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fun {
    pub name: Name,
    pub producers: Vec<Producer>,
    pub consumers: Vec<Consumer>,
}

impl std::fmt::Display for Fun {
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
        write!(f, "{}({};{})", self.name, args_joined, coargs_joined)
    }
}

impl From<Fun> for Statement {
    fn from(value: Fun) -> Self {
        Statement::Fun(value)
    }
}

impl FreeV for Fun {
    fn free_vars(&self) -> HashSet<Var> {
        let free_p = self.producers.free_vars();
        let free_c = self.consumers.free_vars();
        free_p.union(&free_c).cloned().collect()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let free_p = self.producers.free_covars();
        let free_c = self.consumers.free_covars();
        free_p.union(&free_c).cloned().collect()
    }
}
impl Subst for Fun {
    type Target = Fun;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        Fun {
            name: self.name.clone(),
            producers: self.producers.subst_sim(prod_subst, cons_subst),
            consumers: self.consumers.subst_sim(prod_subst, cons_subst),
        }
    }
}
