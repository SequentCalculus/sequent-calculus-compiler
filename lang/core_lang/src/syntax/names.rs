//! This module defines some utilities to deal with names and lists of names.

use crate::traits::*;
use printer::*;

use std::fmt;

/// Identifier used in the program
/// ids are globally unique (after the [`Uniquify`] pass)
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ident {
    /// base name of the Ident
    pub name: String,
    /// unique id
    pub id: usize,
}

impl Ident {
    /// Create a new identifier with id `0`
    pub fn new_with_zero(base_name: &str) -> Self {
        Self {
            name: base_name.to_string(),
            id: 0,
        }
    }
}

/// Create a new ident with given base name and last used id
pub fn fresh_ident(max_id: &mut usize, base_name: &str) -> Ident {
    let new_ident = Ident {
        name: base_name.to_string(),
        id: *max_id + 1,
    };
    *max_id += 1;
    new_ident
}

/// Create a new variable `xi`
pub fn fresh_var(max_id: &mut usize) -> Ident {
    fresh_ident(max_id, "x")
}

/// Create a new covariable `ai`
pub fn fresh_covar(max_id: &mut usize) -> Ident {
    fresh_ident(max_id, "a")
}

impl SubstVar for Ident {
    type Target = Ident;
    fn subst_sim(self, subst: &[(Ident, Ident)]) -> Ident {
        match subst.iter().find(|(old, _)| *old == self) {
            None => self,
            Some((_, new)) => new.clone(),
        }
    }
}

impl Print for Ident {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        if self.id == 0 {
            self.name.print(cfg, alloc)
        } else {
            self.name.print(cfg, alloc).append(self.id.to_string())
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
