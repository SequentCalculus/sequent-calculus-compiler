use super::{Covar, Statement, Var};
use crate::{
    syntax::{
        term::{Cns, Prd, Term},
        types::Ty,
    },
    traits::{
        focus::{Bind, Continuation, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
        typed::Typed,
    },
};
use std::{collections::HashSet, fmt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SubstitutionBinding {
    ProducerBinding(Term<Prd>),
    ConsumerBinding(Term<Cns>),
}

impl Typed for SubstitutionBinding {
    fn get_type(&self) -> Ty {
        match self {
            SubstitutionBinding::ProducerBinding(t) => t.get_type(),
            SubstitutionBinding::ConsumerBinding(t) => t.get_type(),
        }
    }
}

pub type Substitution = Vec<SubstitutionBinding>;

impl fmt::Display for SubstitutionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubstitutionBinding::ProducerBinding(t) => t.fmt(f),
            SubstitutionBinding::ConsumerBinding(t) => t.fmt(f),
        }
    }
}

impl FreeV for SubstitutionBinding {
    fn free_vars(&self) -> HashSet<Var> {
        match self {
            SubstitutionBinding::ProducerBinding(t) => t.free_vars(),
            SubstitutionBinding::ConsumerBinding(t) => t.free_vars(),
        }
    }
    fn free_covars(&self) -> HashSet<Covar> {
        match self {
            SubstitutionBinding::ProducerBinding(t) => t.free_covars(),
            SubstitutionBinding::ConsumerBinding(t) => t.free_covars(),
        }
    }
}

impl Subst for SubstitutionBinding {
    type Target = SubstitutionBinding;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        match self {
            SubstitutionBinding::ProducerBinding(t) => {
                SubstitutionBinding::ProducerBinding(t.subst_sim(prod_subst, cons_subst))
            }
            SubstitutionBinding::ConsumerBinding(t) => {
                SubstitutionBinding::ConsumerBinding(t.subst_sim(prod_subst, cons_subst))
            }
        }
    }
}

impl Focusing for SubstitutionBinding {
    type Target = SubstitutionBinding;
    fn focus(self, state: &mut FocusingState) -> Self::Target {
        match self {
            SubstitutionBinding::ProducerBinding(t) => {
                SubstitutionBinding::ProducerBinding(t.focus(state))
            }
            SubstitutionBinding::ConsumerBinding(t) => {
                SubstitutionBinding::ConsumerBinding(t.focus(state))
            }
        }
    }
}

impl Bind for SubstitutionBinding {
    fn bind(self, k: Continuation, state: &mut FocusingState) -> Statement {
        match self {
            SubstitutionBinding::ProducerBinding(t) => t.bind(k, state),
            SubstitutionBinding::ConsumerBinding(t) => t.bind(k, state),
        }
    }
}
