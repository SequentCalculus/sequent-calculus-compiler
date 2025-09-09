//! This module defines arguments in Core.

use printer::*;

use crate::syntax::*;
use crate::traits::*;

use std::collections::{BTreeSet, HashSet, VecDeque};

/// A single argument that can be either a producer or a consumer.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Argument {
    /// A producer argument
    Producer(Term<Prd>),
    /// A consumer argument
    Consumer(Term<Cns>),
}

impl Print for Argument {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        match self {
            Argument::Producer(term) => term.print(cfg, alloc),
            Argument::Consumer(term) => term.print(cfg, alloc),
        }
    }
}

impl From<Term<Prd>> for Argument {
    fn from(prod: Term<Prd>) -> Argument {
        Argument::Producer(prod)
    }
}

impl From<Term<Cns>> for Argument {
    fn from(cons: Term<Cns>) -> Argument {
        Argument::Consumer(cons)
    }
}

impl Subst for Argument {
    type Target = Argument;
    fn subst_sim(
        self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        match self {
            Argument::Producer(prod) => Argument::Producer(prod.subst_sim(prod_subst, cons_subst)),
            Argument::Consumer(cons) => Argument::Consumer(cons.subst_sim(prod_subst, cons_subst)),
        }
    }
}

impl TypedFreeVars for Argument {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        match self {
            Argument::Producer(term) => {
                term.typed_free_vars(vars);
            }
            Argument::Consumer(term) => {
                term.typed_free_vars(vars);
            }
        }
    }
}

impl Uniquify for Argument {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Argument {
        match self {
            Argument::Producer(term) => Argument::Producer(term.uniquify(seen_vars, used_vars)),
            Argument::Consumer(term) => Argument::Consumer(term.uniquify(seen_vars, used_vars)),
        }
    }
}

impl Bind for Argument {
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        match self {
            Argument::Producer(prd) => prd.bind(k, used_vars),
            Argument::Consumer(cns) => cns.bind(k, used_vars),
        }
    }
}

/// This struct defines arguments in Core. They consist of a list of [`ArgumentEntry`]s.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Arguments {
    pub entries: Vec<Argument>,
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

impl From<Arguments> for VecDeque<Argument> {
    fn from(s: Arguments) -> VecDeque<Argument> {
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
