//! This module defines some utilities to deal with names and lists of names.

use crate::traits::substitution::Subst;
use printer::*;

/// Names of top-level functions, user-declared types and xtors.
pub type Name = String;

/// Variables
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Var {
    pub name: String,
    pub id: usize,
}

impl Subst for Var {
    fn subst_sim(self, subst: &[(Var, Var)]) -> Var {
        match subst.iter().find(|(old, _)| *old == self) {
            None => self,
            Some((_, new)) => new.clone(),
        }
    }
}

impl Print for Var {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.name.print(cfg, alloc).append(self.id.to_string())
    }
}
