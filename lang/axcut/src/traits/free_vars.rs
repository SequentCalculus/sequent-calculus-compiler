//! This module defines a trait with a method for computing the free variables of a statement.

use crate::syntax::Ident;

use std::collections::HashSet;
use std::rc::Rc;

/// This trait defines a method for computing the free variables of a statement and annotating the
/// free variables of substatements.
pub trait FreeVars: Sized {
    /// This method calculates the free variables of a statement. It moreover annotates the free
    /// variables for each substatement where necessary to avoid repeated computation of free
    /// variables.
    /// - `vars` is a reference to the set into which the free variables are collected. The set is
    ///   assumed to be empty when passed to this method.
    fn free_vars(self, vars: &mut HashSet<Ident>) -> Self;
}

impl<T: FreeVars + Clone> FreeVars for Rc<T> {
    fn free_vars(self, vars: &mut HashSet<Ident>) -> Self {
        Rc::new(Rc::unwrap_or_clone(self).free_vars(vars))
    }
}

impl<T: FreeVars> FreeVars for Vec<T> {
    fn free_vars(self, vars: &mut HashSet<Ident>) -> Self {
        self.into_iter()
            .map(|element| {
                let mut free_vars = HashSet::new();
                let element = element.free_vars(&mut free_vars);
                vars.extend(free_vars);
                element
            })
            .collect()
    }
}
