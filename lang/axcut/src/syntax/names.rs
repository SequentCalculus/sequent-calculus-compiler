//! This module defines some utilities to deal with names and lists of names.

use crate::traits::substitution::Subst;

use std::collections::HashSet;

/// Names of top-level functions, user-declared types and xtors.
pub type Name = String;
/// Variables
pub type Var = String;

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

impl Subst for Var {
    fn subst_sim(self, subst: &[(Var, Var)]) -> Var {
        match subst.iter().find(|(old, _)| *old == self) {
            None => self,
            Some((_, new)) => new.clone(),
        }
    }
}
