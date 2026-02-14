//! This module defines some utilities to deal with names and lists of names.
use std::collections::HashSet;

/// Type Alias for Variables.
pub type Var = String;
/// Type Alias for Covariables.
pub type Covar = String;
/// Type alias for names of top-level functions, user-declared types and xtors
pub type Name = String;

/// Create a fresh Var/Covar/Name with given base name
pub fn fresh_ident(used: &mut HashSet<String>, base: &str) -> String {
    let mut n = 0;
    let mut new_var = format!("{base}{n}");
    while used.contains(&new_var) {
        n += 1;
        new_var = format!("{base}{n}");
    }
    used.insert(new_var.clone());
    new_var
}
/// Create a unique variable with base name "x"
pub fn fresh_var(used_vars: &mut HashSet<Var>) -> Var {
    fresh_ident(used_vars, "x")
}

/// Create a unique covariable with base name "a"
pub fn fresh_covar(used_covars: &mut HashSet<Covar>) -> Covar {
    fresh_ident(used_covars, "a")
}
