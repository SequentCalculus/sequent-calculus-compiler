//! This module defines some utilities to deal with names and lists of names.

use crate::traits::substitution::Subst;

/// Names of top-level functions, user-declared types and xtors.
pub type Name = String;
/// Variables
pub type Var = String;

impl Subst for Var {
    fn subst_sim(self, subst: &[(Var, Var)]) -> Var {
        match subst.iter().find(|(old, _)| *old == self) {
            None => self,
            Some((_, new)) => new.clone(),
        }
    }
}
