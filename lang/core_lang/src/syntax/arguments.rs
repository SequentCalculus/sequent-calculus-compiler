//! This module defines arguments in Core.

use printer::*;

use crate::syntax::*;
use crate::traits::*;

use std::collections::{BTreeSet, VecDeque};

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

impl Typed for Argument {
    fn get_type(&self) -> Ty {
        match self {
            Argument::Producer(prd) => prd.get_type(),
            Argument::Consumer(cns) => cns.get_type(),
        }
    }
}

impl IsCoValue for Argument {
    fn is_co_value(&self, codata_types: &[CodataDeclaration]) -> bool {
        match self {
            Argument::Producer(prd) => prd.is_value(codata_types),
            Argument::Consumer(cns) => cns.is_covalue(codata_types),
        }
    }
}

impl Subst for Argument {
    type Target = Argument;
    fn subst_sim(
        self,
        prod_subst: &[(Identifier, Term<Prd>)],
        cons_subst: &[(Identifier, Term<Cns>)],
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
    fn uniquify(self, max_id: &mut ID) -> Argument {
        match self {
            Argument::Producer(term) => Argument::Producer(term.uniquify(max_id)),
            Argument::Consumer(term) => Argument::Consumer(term.uniquify(max_id)),
        }
    }
}

impl Bind for Argument {
    fn bind(self, k: Continuation, max_id: &mut ID) -> FsStatement {
        match self {
            Argument::Producer(prd) => prd.bind(k, max_id),
            Argument::Consumer(cns) => cns.bind(k, max_id),
        }
    }
}

/// This struct defines arguments in Core. They consist of a list of [`Argument`]s.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Arguments {
    pub entries: Vec<Argument>,
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
    fn from(args: Arguments) -> VecDeque<Argument> {
        args.entries.into()
    }
}

impl From<VecDeque<Argument>> for Arguments {
    fn from(args: VecDeque<Argument>) -> Arguments {
        Arguments {
            entries: args.into(),
        }
    }
}

impl IsCoValue for Arguments {
    fn is_co_value(&self, codata_types: &[CodataDeclaration]) -> bool {
        self.entries.is_co_value(codata_types)
    }
}

impl Subst for Arguments {
    type Target = Arguments;
    fn subst_sim(
        mut self,
        prod_subst: &[(Identifier, Term<Prd>)],
        cons_subst: &[(Identifier, Term<Cns>)],
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
    fn uniquify(mut self, max_id: &mut ID) -> Arguments {
        self.entries = self.entries.uniquify(max_id);
        self
    }
}
