use crate::syntax::ContextBinding;
use std::collections::BTreeSet;

pub trait TypedFreeVars {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding>;
}
