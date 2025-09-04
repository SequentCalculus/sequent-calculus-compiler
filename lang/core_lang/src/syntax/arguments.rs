//! This module defines arguments in Core.

use printer::{DocAllocator, Print};

use super::{ContextBinding, Covar, Var};
use crate::{
    syntax::{
        FsStatement,
        terms::{Cns, Prd, Term},
    },
    traits::*,
};

use std::collections::{BTreeSet, HashSet, VecDeque};

/// This struct defines an argument entry in Core. It is either a procuder or a consumer.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ArgumentEntry {
    /// Producer
    ProducerEntry(Term<Prd>),
    /// Consumer
    ConsumerEntry(Term<Cns>),
}

impl Print for ArgumentEntry {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            ArgumentEntry::ProducerEntry(term) => term.print(cfg, alloc),
            ArgumentEntry::ConsumerEntry(term) => term.print(cfg, alloc),
        }
    }
}

impl From<Term<Prd>> for ArgumentEntry {
    fn from(prod: Term<Prd>) -> ArgumentEntry {
        ArgumentEntry::ProducerEntry(prod)
    }
}

impl From<Term<Cns>> for ArgumentEntry {
    fn from(cons: Term<Cns>) -> ArgumentEntry {
        ArgumentEntry::ConsumerEntry(cons)
    }
}

impl Subst for ArgumentEntry {
    type Target = ArgumentEntry;
    fn subst_sim(
        self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        match self {
            ArgumentEntry::ProducerEntry(prod) => {
                ArgumentEntry::ProducerEntry(prod.subst_sim(prod_subst, cons_subst))
            }
            ArgumentEntry::ConsumerEntry(cons) => {
                ArgumentEntry::ConsumerEntry(cons.subst_sim(prod_subst, cons_subst))
            }
        }
    }
}

impl TypedFreeVars for ArgumentEntry {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        match self {
            ArgumentEntry::ProducerEntry(term) => {
                term.typed_free_vars(vars);
            }
            ArgumentEntry::ConsumerEntry(term) => {
                term.typed_free_vars(vars);
            }
        }
    }
}

impl Uniquify for ArgumentEntry {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> ArgumentEntry {
        match self {
            ArgumentEntry::ProducerEntry(term) => {
                ArgumentEntry::ProducerEntry(term.uniquify(seen_vars, used_vars))
            }
            ArgumentEntry::ConsumerEntry(term) => {
                ArgumentEntry::ConsumerEntry(term.uniquify(seen_vars, used_vars))
            }
        }
    }
}

impl Bind for ArgumentEntry {
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        match self {
            ArgumentEntry::ProducerEntry(prd) => prd.bind(k, used_vars),
            ArgumentEntry::ConsumerEntry(cns) => cns.bind(k, used_vars),
        }
    }
}

/// This struct defines arguments in Core. They consist of a list of [`ArgumentEntry`]s.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Arguments {
    pub entries: Vec<ArgumentEntry>,
}

impl Arguments {
    /// This fucntion adds a producer term to the arguments.
    pub fn add_prod<T: Into<Term<Prd>>>(&mut self, t: T) {
        self.entries.push(t.into().into());
    }

    /// This function adds a consumer term to the arguments.
    pub fn add_cons<T: Into<Term<Cns>>>(&mut self, t: T) {
        self.entries.push(t.into().into());
    }
}

impl Print for Arguments {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let sep = if cfg.allow_linebreaks {
            alloc.line_()
        } else {
            alloc.nil()
        };

        if self.entries.is_empty() {
            alloc.nil()
        } else {
            sep.clone()
                .append(self.entries.print(cfg, alloc))
                .nest(cfg.indent)
                .append(sep)
        }
    }
}

impl From<Arguments> for VecDeque<ArgumentEntry> {
    fn from(s: Arguments) -> VecDeque<ArgumentEntry> {
        s.entries.into()
    }
}

impl Subst for Arguments {
    type Target = Arguments;
    fn subst_sim(
        mut self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        self.entries = self.entries.subst_sim(prod_subst, cons_subst);
        self
    }
}

impl TypedFreeVars for Arguments {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.entries.typed_free_vars(vars);
    }
}

impl Uniquify for Arguments {
    fn uniquify(mut self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Arguments {
        self.entries = self.entries.uniquify(seen_vars, used_vars);
        self
    }
}
