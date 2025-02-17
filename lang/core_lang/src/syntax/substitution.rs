use printer::Print;

use super::{Covar, Var};
use crate::{
    syntax::{
        terms::{Cns, Prd, Term},
        FsStatement,
    },
    traits::*,
};

use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SubstitutionBinding {
    ProducerBinding(Term<Prd>),
    ConsumerBinding(Term<Cns>),
}

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

impl Subst for SubstitutionBinding {
    type Target = SubstitutionBinding;
    fn subst_sim(
        self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
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

impl Bind for SubstitutionBinding {
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        match self {
            SubstitutionBinding::ProducerBinding(prd) => prd.bind(k, used_vars),
            SubstitutionBinding::ConsumerBinding(cns) => cns.bind(k, used_vars),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Substitution(pub Vec<SubstitutionBinding>);

impl Substitution {
    pub fn add_prod<T: Into<Term<Prd>>>(&mut self, t: T) {
        self.0.push(t.into().into());
    }

    pub fn add_cons<T: Into<Term<Cns>>>(&mut self, t: T) {
        self.0.push(t.into().into());
    }
}

impl Print for Substitution {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.0.print(cfg, alloc)
    }
}

impl From<Substitution> for VecDeque<SubstitutionBinding> {
    fn from(s: Substitution) -> VecDeque<SubstitutionBinding> {
        s.0.into()
    }
}

impl Subst for Substitution {
    type Target = Substitution;
    fn subst_sim(
        self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        Substitution(self.0.subst_sim(prod_subst, cons_subst))
    }
}

impl Uniquify for Substitution {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Substitution {
        Substitution(self.0.uniquify(seen_vars, used_vars))
    }
}
