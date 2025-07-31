//! This module defines a trait with a method for computing the free variables of a term or
//! statement, including their types.

use crate::syntax::ContextBinding;

use std::collections::BTreeSet;
use std::rc::Rc;

/// This trait defines a method for computing the free variables of a term or statement, including
/// their types.
pub trait TypedFreeVars: Sized {
    /// This method calculates the free variables of a term or statement, including their types.
    /// - `vars` is a reference to the ordered set into which the free variables are collected.
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>);
}

impl<T: TypedFreeVars> TypedFreeVars for Rc<T> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        (**self).typed_free_vars(vars);
    }
}

impl<T: TypedFreeVars> TypedFreeVars for Option<T> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        match self {
            None => {}
            Some(t) => t.typed_free_vars(vars),
        }
    }
}

impl<T: TypedFreeVars> TypedFreeVars for Vec<T> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        for element in self {
            element.typed_free_vars(vars);
        }
    }
}
