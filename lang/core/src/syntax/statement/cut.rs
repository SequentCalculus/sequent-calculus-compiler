use super::{Consumer, Covar, Producer, Statement, Var};
use crate::{
    syntax::term::{Cns, Prd, Term},
    traits::{free_vars::FreeV, substitution::Subst},
};
use std::{collections::HashSet, fmt, rc::Rc};

// Cut
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cut {
    pub producer: Rc<Producer>,
    pub consumer: Rc<Consumer>,
}

impl std::fmt::Display for Cut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Cut { producer, consumer } = self;
        write!(f, "<{} | {}>", producer, consumer)
    }
}

impl FreeV for Cut {
    fn free_vars(&self) -> HashSet<Var> {
        let Cut { producer, consumer } = self;
        let mut free_vars = producer.free_vars();
        free_vars.extend(consumer.free_vars());
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let Cut { producer, consumer } = self;
        let mut free_covars = producer.free_covars();
        free_covars.extend(consumer.free_covars());
        free_covars
    }
}

impl From<Cut> for Statement {
    fn from(value: Cut) -> Self {
        Statement::Cut(value)
    }
}

impl Subst for Cut {
    type Target = Cut;

    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        Cut {
            producer: self.producer.subst_sim(prod_subst, cons_subst),
            consumer: self.consumer.subst_sim(prod_subst, cons_subst),
        }
    }
}
