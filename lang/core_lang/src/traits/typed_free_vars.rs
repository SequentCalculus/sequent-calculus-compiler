use crate::syntax::ContextBinding;

use std::collections::BTreeSet;

/// Computing the typed free variables of a term.
pub trait TypedFreeVars: Sized {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>);
}

impl<T: TypedFreeVars> TypedFreeVars for Vec<T> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        for element in self {
            element.typed_free_vars(vars);
        }
    }
}
