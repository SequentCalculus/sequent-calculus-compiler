//! This module defines some utilities to deal with names and lists of names.

use crate::traits::*;

use std::collections::HashSet;

/// Type alias for variables
pub type Var = String;
/// Type alias for covariables
pub type Covar = String;
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
pub fn fresh_var(used_vars: &mut HashSet<Var>) -> Var {
    fresh_name(used_vars, "x")
}

/// This function generates a fresh covariable with base name `"a"`.
pub fn fresh_covar(used_covars: &mut HashSet<Covar>) -> Covar {
    fresh_name(used_covars, "a")
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
