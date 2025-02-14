use crate::traits::*;

use std::collections::HashSet;

pub type Var = String;
pub type Covar = String;
pub type Name = String;

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

pub fn fresh_var(used_vars: &mut HashSet<Var>) -> Var {
    fresh_name(used_vars, "x")
}

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
