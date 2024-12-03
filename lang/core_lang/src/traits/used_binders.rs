use std::collections::HashSet;

use crate::syntax::Var;

pub trait UsedBinders {
    fn used_binders(&self, used: &mut HashSet<Var>);
}

impl<T: UsedBinders> UsedBinders for Vec<T> {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        for element in self {
            element.used_binders(used);
        }
    }
}
