//! Names in the core language
use crate::traits::*;

use std::collections::HashSet;

/// Type alias for variables
pub type Var = String;
/// Type alias for covariables
pub type Covar = String;
/// Type alias for names (of types and definitions)
pub type Name = String;

/// Generate a fresh name that has not been used before with a given prefix
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

/// Generate a fresh variable
pub fn fresh_var(used_vars: &mut HashSet<Var>) -> Var {
    fresh_name(used_vars, "x")
}

/// Generate a fresh covariable
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
