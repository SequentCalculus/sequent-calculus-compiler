use printer::Print;

use super::{Covar, Var};
use crate::{
    syntax::{
        term::{Cns, Prd, Term},
        FsStatement,
    },
    traits::*,
};

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SubstitutionBinding {
    ProducerBinding(Term<Prd>),
    ConsumerBinding(Term<Cns>),
}

pub type Substitution = Vec<SubstitutionBinding>;

impl Print for SubstitutionBinding {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            SubstitutionBinding::ProducerBinding(term) => term.print(cfg, alloc),
            SubstitutionBinding::ConsumerBinding(term) => term.print(cfg, alloc),
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

impl UsedBinders for SubstitutionBinding {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        match self {
            SubstitutionBinding::ProducerBinding(prd) => prd.used_binders(used),
            SubstitutionBinding::ConsumerBinding(cns) => cns.used_binders(used),
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

impl Uniquify for SubstitutionBinding {
    fn uniquify(
        self,
        seen_vars: &mut HashSet<Var>,
        used_vars: &mut HashSet<Var>,
    ) -> SubstitutionBinding {
        match self {
            SubstitutionBinding::ProducerBinding(term) => {
                SubstitutionBinding::ProducerBinding(term.uniquify(seen_vars, used_vars))
            }
            SubstitutionBinding::ConsumerBinding(term) => {
                SubstitutionBinding::ConsumerBinding(term.uniquify(seen_vars, used_vars))
            }
        }
    }
}

impl Focusing for SubstitutionBinding {
    type Target = SubstitutionBinding;
    fn focus(self, _state: &mut FocusingState) -> Self::Target {
        panic!("Focusing should never be called directly on a substitution binding");
    }
}

impl Bind for SubstitutionBinding {
    fn bind(self, k: Continuation, state: &mut FocusingState) -> FsStatement {
        match self {
            SubstitutionBinding::ProducerBinding(prd) => prd.bind(k, state),
            SubstitutionBinding::ConsumerBinding(cns) => cns.bind(k, state),
        }
    }
}
