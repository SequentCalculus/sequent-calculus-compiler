//! This module defines some utilities to deal with names and lists of names.

use crate::traits::*;
use printer::*;

use std::{collections::HashSet, fmt};

/// Type alias for variables
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Var {
    pub name: Name,
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

pub fn fresh_xvar(used_vars: &mut HashSet<Var>, name: &str) -> Var {
    let new_id = used_vars
        .iter()
        .filter_map(|var| (var.name == name).then_some(var.id))
        .max()
        .unwrap_or(0);
    let new_var = Var {
        name: name.to_string(),
        id: new_id,
    };
    used_vars.insert(new_var.clone());
    new_var
}

/// This function generates a fresh variable with base name `"x"`.
pub fn fresh_var(used_vars: &mut HashSet<Var>) -> Var {
    fresh_xvar(used_vars, "x")
}

/// This function generates a fresh covariable with base name `"a"`.
pub fn fresh_covar(used_covars: &mut HashSet<Var>) -> Var {
    fresh_xvar(used_covars, "a")
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
        self.name.print(cfg, alloc)
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
