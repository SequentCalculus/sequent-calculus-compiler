use super::{Case, Covar, Covariable, Destructor, MuTilde, Producer, Var};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt};

// Consumer
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Consumer {
    Covariable(Covariable),
    MuTilde(MuTilde),
    Case(Case),
    Destructor(Destructor),
}

impl std::fmt::Display for Consumer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Consumer::Covariable(cv) => cv.fmt(f),
            Consumer::MuTilde(m) => m.fmt(f),
            Consumer::Case(case) => case.fmt(f),
            Consumer::Destructor(d) => d.fmt(f),
        }
    }
}

impl FreeV for Consumer {
    fn free_vars(self: &Consumer) -> HashSet<Var> {
        match self {
            Consumer::Covariable(cv) => cv.free_vars(),
            Consumer::MuTilde(m) => m.free_vars(),
            Consumer::Case(pts) => pts.free_vars(),
            Consumer::Destructor(d) => d.free_vars(),
        }
    }

    fn free_covars(self: &Consumer) -> HashSet<Covar> {
        match self {
            Consumer::Covariable(covar) => covar.free_covars(),
            Consumer::MuTilde(m) => m.free_covars(),
            Consumer::Case(c) => c.free_covars(),
            Consumer::Destructor(d) => d.free_covars(),
        }
    }
}

impl Subst for Consumer {
    type Target = Consumer;
    fn subst_sim(
        self: &Consumer,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Consumer {
        match self {
            Consumer::Covariable(covar) => covar.subst_sim(prod_subst, cons_subst),
            Consumer::MuTilde(m) => m.subst_sim(prod_subst, cons_subst).into(),
            Consumer::Case(c) => c.subst_sim(prod_subst, cons_subst).into(),
            Consumer::Destructor(d) => d.subst_sim(prod_subst, cons_subst).into(),
        }
    }
}
