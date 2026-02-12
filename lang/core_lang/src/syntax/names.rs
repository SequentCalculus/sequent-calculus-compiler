//! This module defines some utilities to deal with names and lists of names.

use crate::traits::*;
use printer::*;

use std::collections::HashSet;

/// Type for variables and covariables
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Var {
    pub name: String,
    pub id: usize,
}

/// Type alias for names of top-level functions, user-declared types and xtors
pub type Name = String;

/// This function generates a fresh name with respect to a given set of names.
/// - `used_vars` is the set of names for which to generate a fresh one.
/// - `base_name` is the base name for the generated name to which a number is appended that makes
///   it fresh.
pub fn fresh_name(used_names: &mut HashSet<Name>, base_name: &str) -> Name {
    let mut n = 0;
    let mut new_name: Name = format!("{base_name}{n}");
    while used_names.contains(&new_name) {
        n += 1;
        new_name = format!("{base_name}{n}");
    }
    used_names.insert(new_name.clone());
    new_name
}

/// This function generates a fresh variable with base name `"x"`.
pub fn fresh_var(used_vars: &mut HashSet<Var>, base_name: &str) -> Var {
    let mut new_var = Var {
        name: base_name.to_string(),
        id: 0,
    };
    while used_vars.contains(&new_var) {
        new_var.id += 1;
    }
    used_vars.insert(new_var.clone());
    new_var
}

impl SubstVar for Var {
    type Target = Var;
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
