//! This module defines a trait for collecting the names of all binders used in a given term.
//!
use crate::syntax::names::Var;

use std::collections::HashSet;
use std::rc::Rc;

/// This trait provides a method for for collecting the names of all binders used in a given term.
pub trait UsedBinders {
    /// This method collects the names of all binders used in a given term into a set.
    /// - `used` is the set into which the names are collected.
    fn used_binders(&self, used: &mut HashSet<Var>);
}

impl<T: UsedBinders> UsedBinders for Vec<T> {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        for element in self {
            element.used_binders(used);
        }
    }
}

impl<T: UsedBinders> UsedBinders for Rc<T> {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        (**self).used_binders(used);
    }
}

impl<T: UsedBinders> UsedBinders for Option<T> {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        match self {
            None => {}
            Some(t) => t.used_binders(used),
        }
    }
}
