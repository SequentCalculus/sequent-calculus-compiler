use crate::syntax::Var;

use std::collections::HashSet;
use std::rc::Rc;

/// Trait for terms to collect all bound names
pub trait UsedBinders {
    /// Collects all names bound by &self
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
