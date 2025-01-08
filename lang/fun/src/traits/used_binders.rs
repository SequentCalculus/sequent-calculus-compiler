use std::collections::HashSet;

use crate::syntax::XVar;

pub trait UsedBinders {
    fn used_binders(&self, used: &mut HashSet<XVar>);
}

impl<T: UsedBinders> UsedBinders for Vec<T> {
    fn used_binders(&self, used: &mut HashSet<XVar>) {
        for element in self {
            element.used_binders(used);
        }
    }
}
