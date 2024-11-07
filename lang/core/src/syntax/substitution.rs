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
    ProducerBinding { prd: Term<Prd>, ty: Ty },
    ConsumerBinding { cns: Term<Cns>, ty: Ty },
}

impl Typed for SubstitutionBinding {
    fn get_type(&self) -> Ty {
        match self {
            SubstitutionBinding::ProducerBinding { prd: _, ty } => ty.clone(),
            SubstitutionBinding::ConsumerBinding { cns: _, ty } => ty.clone(),
        }
    }
}

pub type Substitution = Vec<SubstitutionBinding>;

impl fmt::Display for SubstitutionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubstitutionBinding::ProducerBinding { prd, ty: _ } => prd.fmt(f),
            SubstitutionBinding::ConsumerBinding { cns, ty: _ } => cns.fmt(f),
        }
    }
}

impl FreeV for SubstitutionBinding {
    fn free_vars(&self) -> HashSet<Var> {
        match self {
            SubstitutionBinding::ProducerBinding { prd, ty: _ } => prd.free_vars(),
            SubstitutionBinding::ConsumerBinding { cns, ty: _ } => cns.free_vars(),
        }
    }
    fn free_covars(&self) -> HashSet<Covar> {
        match self {
            SubstitutionBinding::ProducerBinding { prd, ty: _ } => prd.free_covars(),
            SubstitutionBinding::ConsumerBinding { cns, ty: _ } => cns.free_covars(),
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
            SubstitutionBinding::ProducerBinding { prd, ty } => {
                SubstitutionBinding::ProducerBinding {
                    prd: prd.subst_sim(prod_subst, cons_subst),
                    ty: ty.clone(),
                }
            }
            SubstitutionBinding::ConsumerBinding { cns, ty } => {
                SubstitutionBinding::ConsumerBinding {
                    cns: cns.subst_sim(prod_subst, cons_subst),
                    ty: ty.clone(),
                }
            }
        }
    }
}

impl Focusing for SubstitutionBinding {
    type Target = SubstitutionBinding;
    fn focus(self, state: &mut FocusingState) -> Self::Target {
        match self {
            SubstitutionBinding::ProducerBinding { prd, ty } => {
                SubstitutionBinding::ProducerBinding {
                    prd: prd.focus(state),
                    ty: ty.clone(),
                }
            }
            SubstitutionBinding::ConsumerBinding { cns, ty } => {
                SubstitutionBinding::ConsumerBinding {
                    cns: cns.focus(state),
                    ty: ty.clone(),
                }
            }
        }
    }
}

impl Bind for SubstitutionBinding {
    fn bind(self, k: Continuation, state: &mut FocusingState) -> Statement {
        match self {
            SubstitutionBinding::ProducerBinding { prd, ty: _ } => prd.bind(k, state),
            SubstitutionBinding::ConsumerBinding { cns, ty: _ } => cns.bind(k, state),
        }
    }
}
