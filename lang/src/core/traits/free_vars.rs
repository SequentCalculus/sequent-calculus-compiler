use std::collections::HashSet;
use std::rc::Rc;

use crate::core::syntax::{Covariable, Var};

/// Computing the free variables and covariables of a term.
pub trait FreeV {
    /// Compute the free variables of a term.
    fn free_vars(&self) -> HashSet<Var>;
    /// Compute the free covariables of a term.
    fn free_covars(&self) -> HashSet<Covariable>;
}

impl<T: FreeV> FreeV for Vec<T> {
    fn free_vars(self: &Vec<T>) -> HashSet<Var> {
        self.iter().fold(HashSet::new(), |free_vars, t| {
            free_vars.union(&t.free_vars()).cloned().collect()
        })
    }
    fn free_covars(self: &Vec<T>) -> HashSet<Covariable> {
        self.iter().fold(HashSet::new(), |free_vars, t| {
            free_vars.union(&t.free_vars()).cloned().collect()
        })
    }
}

impl<T: FreeV> FreeV for Rc<T> {
    fn free_vars(&self) -> HashSet<Var> {
        (**self).free_vars()
    }

    fn free_covars(&self) -> HashSet<Covariable> {
        (**self).free_covars()
    }
}

pub fn fresh_var(xs: &HashSet<Var>) -> Var {
    fresh_var_n(xs, 0)
}

fn fresh_var_n(xs: &HashSet<Var>, n: i32) -> Var {
    let new_var: Var = format!("x{}", n);
    if xs.contains(&new_var) {
        fresh_var_n(xs, n + 1)
    } else {
        new_var
    }
}

pub fn fresh_covar(xs: &HashSet<Covariable>) -> Covariable {
    fresh_covar_n(xs, 0)
}

fn fresh_covar_n(xs: &HashSet<Covariable>, n: i32) -> Covariable {
    let new_covar: Covariable = format!("a{}", n);
    if xs.contains(&new_covar) {
        fresh_covar_n(xs, n + 1)
    } else {
        new_covar
    }
}
