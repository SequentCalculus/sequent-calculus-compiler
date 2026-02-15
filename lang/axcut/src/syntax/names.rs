//! This module defines some utilities to deal with names and lists of names.

use crate::traits::substitution::Subst;
use printer::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ident {
    pub name: String,
    pub id: usize,
}

impl Subst for Ident {
    fn subst_sim(self, subst: &[(Ident, Ident)]) -> Ident {
        match subst.iter().find(|(old, _)| *old == self) {
            None => self,
            Some((_, new)) => new.clone(),
        }
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.id == 0 {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{}{}", self.name, self.id)
        }
    }
}
impl Print for Ident {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.id == 0 {
            self.name.print(cfg, alloc)
        } else {
            self.name.print(cfg, alloc).append(self.id.to_string())
        }
    }
}
