//! This module defines some utilities to deal with names and lists of names.
use std::collections::HashSet;

/// Type Alias for Variables.
pub type Var = String;
/// Type Alias for Covariables.
pub type Covar = String;
/// Type alias for names. This is used in particular for top-level functions, user-declared types,
/// and xtors of those types.
pub type Name = String;

/// Create a fresh Name with given base name
pub fn fresh_name(used: &mut HashSet<Name>, base: &str) -> String {
    let mut n = 0;
    let mut new_name = format!("{base}{n}");
    while used.contains(&new_name) {
        n += 1;
        new_name = format!("{base}{n}");
    }
    used.insert(new_name.clone());
    new_name
}

/// Create a fresh variable with base name "x"
pub fn fresh_var(used_vars: &mut HashSet<Var>) -> Var {
    fresh_name(used_vars, "x")
}

/// Create a fresh covariable with base name "a"
pub fn fresh_covar(used_covars: &mut HashSet<Covar>) -> Covar {
    fresh_name(used_covars, "a")
}
