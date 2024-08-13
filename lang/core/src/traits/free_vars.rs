use std::collections::HashSet;

use crate::syntax::{Covar, Var};

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
        self.iter().fold(HashSet::new(), |mut free_vars, t| {
            free_vars.extend(t.free_vars());
            free_vars
        })
    }
}

pub fn fresh_var(xs: &HashSet<Var>) -> Var {
    fresh_var_n(xs, 0)
}

fn fresh_var_n(xs: &HashSet<Var>, mut n: i32) -> Var {
    let mut new_var: Var = format!("x{}", n);
    while xs.contains(&new_var) {
        n += 1;
        new_var = format!("x{}", n);
    }
    new_var
}

pub fn fresh_covar(xs: &HashSet<Covar>) -> Covar {
    fresh_covar_n(xs, 0)
}

fn fresh_covar_n(xs: &HashSet<Covar>, mut n: i32) -> Covar {
    let mut new_covar: Covar = format!("a{}", n);
    while xs.contains(&new_covar) {
        n += 1;
        new_covar = format!("a{}", n);
    }
    new_covar
}
