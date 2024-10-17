use super::{Covar, Statement, Var};
use crate::{
    syntax::term::{Cns, Prd, Term},
    traits::{
        focus::{Bind, Continuation, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
    },
};
use std::{collections::HashSet, fmt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SubstitutionBinding {
    ProducerBinding(Term<Prd>),
    ConsumerBinding(Term<Cns>),
}

pub type Substitution = Vec<SubstitutionBinding>;

impl fmt::Display for SubstitutionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubstitutionBinding::ProducerBinding(prod) => prod.fmt(f),
            SubstitutionBinding::ConsumerBinding(cons) => cons.fmt(f),
        }
    }
}

impl From<Term<Prd>> for SubstitutionBinding {
    fn from(prod: Term<Prd>) -> SubstitutionBinding {
        SubstitutionBinding::ProducerBinding(prod)
    }
}

impl From<Term<Cns>> for SubstitutionBinding {
    fn from(cons: Term<Cns>) -> SubstitutionBinding {
        SubstitutionBinding::ConsumerBinding(cons)
    }
}

impl FreeV for SubstitutionBinding {
    fn free_vars(&self) -> HashSet<Var> {
        match self {
            SubstitutionBinding::ProducerBinding(prod) => prod.free_vars(),
            SubstitutionBinding::ConsumerBinding(cons) => cons.free_vars(),
        }
    }
    fn free_covars(&self) -> HashSet<Covar> {
        match self {
            SubstitutionBinding::ProducerBinding(prod) => prod.free_covars(),
            SubstitutionBinding::ConsumerBinding(cons) => cons.free_covars(),
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
            SubstitutionBinding::ProducerBinding(prod) => {
                SubstitutionBinding::ProducerBinding(prod.subst_sim(prod_subst, cons_subst))
            }
            SubstitutionBinding::ConsumerBinding(cons) => {
                SubstitutionBinding::ConsumerBinding(cons.subst_sim(prod_subst, cons_subst))
            }
        }
    }
}

impl Focusing for SubstitutionBinding {
    type Target = SubstitutionBinding;
    fn focus(self, state: &mut FocusingState) -> Self::Target {
        match self {
            SubstitutionBinding::ProducerBinding(prod) => {
                SubstitutionBinding::ProducerBinding(prod.focus(state))
            }
            SubstitutionBinding::ConsumerBinding(cons) => {
                SubstitutionBinding::ConsumerBinding(cons.focus(state))
            }
        }
    }
}

impl Bind for SubstitutionBinding {
    fn bind(self, k: Continuation, state: &mut FocusingState) -> Statement {
        match self {
            SubstitutionBinding::ProducerBinding(prod) => prod.bind(k, state),
            SubstitutionBinding::ConsumerBinding(cons) => cons.bind(k, state),
        }
    }
}
