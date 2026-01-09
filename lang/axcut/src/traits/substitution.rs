//! This module defines a trait with a method for substituting a list of variables for other
//! variables in a statement.

use std::collections::HashSet;
use std::rc::Rc;

use crate::syntax::Var;

/// This trait defines a method for substituting a list of variables for other variables in a
/// statement.
pub trait Subst: Clone {
    /// This method substitutes a list of variables for other variables in a statement, also
    /// updating potential annotations of free variables. It assumes all variable bindings in each
    /// path through a statement to be unique, so no care is needed to account for shadowing. It
    /// further assumes that all variables substituted into the statement are fresh for this
    /// statement, so that no care is needed to avoid capture.
    /// - `subst` is the list of substitutions to perform. Each substitution is represented by a
    ///   pair with the first component being the old variable substituted by the new variable in
    ///   the second component.
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self;
}

impl<T: Subst> Subst for Rc<T> {
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self {
        Rc::new(Rc::unwrap_or_clone(self).subst_sim(subst))
    }
}

impl<T: Subst> Subst for Vec<T> {
    fn subst_sim(self, subst: &[(Var, Var)]) -> Vec<T> {
        self.into_iter()
            .map(|element| element.subst_sim(subst))
            .collect()
    }
}

impl<T: Subst + std::hash::Hash + Eq> Subst for HashSet<T> {
    fn subst_sim(self, subst: &[(Var, Var)]) -> HashSet<T> {
        self.into_iter()
            .map(|element| element.subst_sim(subst))
            .collect()
    }
}

impl<T: Subst> Subst for Option<T> {
    fn subst_sim(self, subst: &[(Var, Var)]) -> Option<T> {
        self.map(|t| t.subst_sim(subst))
    }
}
