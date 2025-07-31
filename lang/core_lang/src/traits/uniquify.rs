//! This module defines a trait for making all binders in every path through a term or statement
//! unique.

use crate::syntax::Var;
use std::collections::HashSet;
use std::rc::Rc;

/// This trait defines a method for making all binders in every path through a term or statement
/// unique.
pub trait Uniquify {
    /// This method makes all binders in every path through a term or statement unique by renaming
    /// them if needed.
    /// - `seen_vars` is the set of names we have already seen in the path we are currently in.
    /// - `used_vars` is the set of names used in the whole top-level definition being uniquified.
    ///   It is threaded through the uniquification to facilitate generation of fresh
    ///   (co)variables.
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Self;
}

impl<T: Uniquify + Clone> Uniquify for Rc<T> {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Self {
        Rc::new(Rc::unwrap_or_clone(self).uniquify(seen_vars, used_vars))
    }
}

impl<T: Uniquify> Uniquify for Option<T> {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Self {
        self.map(|t| t.uniquify(seen_vars, used_vars))
    }
}

impl<T: Uniquify> Uniquify for Vec<T> {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Self {
        self.into_iter()
            .map(|element| element.uniquify(seen_vars, used_vars))
            .collect()
    }
}
