//! This module defines some utilities to deal with names and lists of names.

use crate::traits::*;
use printer::*;

use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ident {
    pub name: String,
    id: usize,
}

impl Ident {
    pub fn new_with_zero(base_name: &str) -> Self {
        Self {
            name: base_name.to_string(),
            id: 0,
        }
    }
}

/// This function generates a fresh name with respect to a given set of names.
/// - `used_vars` is the set of names for which to generate a fresh one.
/// - `base_name` is the base name for the generated name to which a number is appended that makes
///   it fresh.
pub fn fresh_name(used_names: &mut HashSet<Ident>, base_name: &str) -> Ident {
    let mut new_name: Ident = Ident {
        name: base_name.to_string(),
        id: 0,
    };
    while used_names.contains(&new_name) {
        new_name.id += 1;
    }
    used_names.insert(new_name.clone());
    new_name
}

/// This function generates a fresh variable with base name `"x"`.
pub fn fresh_var(used_vars: &mut HashSet<Ident>) -> Ident {
    fresh_name(used_vars, "x")
}

/// This function generates a fresh covariable with base name `"a"`.
pub fn fresh_covar(used_covars: &mut HashSet<Ident>) -> Ident {
    fresh_name(used_covars, "a")
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
