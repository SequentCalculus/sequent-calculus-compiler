//! This module defines a trait with a method for computing the free variables of a statement,
//! including their types.

use crate::syntax::ContextBinding;

use std::collections::BTreeSet;

/// This trait defines a method for computing the free variables of a term or statement, including
/// their types.
pub trait TypedFreeVars: Sized {
    /// This method calculates the free variables of a term or statement, including their types.
    /// - `vars` is a reference to the ordered set into which the free variables are collected.
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>);
}

impl<T: TypedFreeVars> TypedFreeVars for Vec<T> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        for element in self {
            element.typed_free_vars(vars);
        }
    }
}
