/// Defines the [TypedFreeVars] trait
use crate::syntax::ContextBinding;

use std::collections::BTreeSet;
use std::rc::Rc;

/// Computing the typed free variables of a term.
pub trait TypedFreeVars: Sized {
    /// Get all free variables
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
