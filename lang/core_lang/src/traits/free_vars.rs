use crate::syntax::{Covar, Var};

use std::collections::HashSet;

/// Computing the free variables and covariables of a term.
pub trait FreeV {
    /// Compute the free variables of a term.
    fn free_vars(&self) -> HashSet<Var>;
    /// Compute the free covariables of a term.
    fn free_covars(&self) -> HashSet<Covar>;
}

impl<T: FreeV> FreeV for Vec<T> {
    fn free_vars(self: &Vec<T>) -> HashSet<Var> {
        self.iter().fold(HashSet::new(), |mut free_vars, t| {
            free_vars.extend(t.free_vars());
            free_vars
        })
    }
    fn free_covars(self: &Vec<T>) -> HashSet<Covar> {
        self.iter().fold(HashSet::new(), |mut free_covars, t| {
            free_covars.extend(t.free_covars());
            free_covars
        })
    }
}

#[must_use]
pub fn fresh_var(used_vars: &mut HashSet<Var>, base_name: &str) -> Var {
    let mut n = 0;
    let mut new_var: Var = format!("{base_name}{n}");
    while used_vars.contains(&new_var) {
        n += 1;
        new_var = format!("{base_name}{n}");
    }
    used_vars.insert(new_var.clone());
    new_var
}
