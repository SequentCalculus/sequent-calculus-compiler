use crate::syntax::Variable;

use std::collections::HashSet;

pub trait UsedBinders {
    fn used_binders(&self, used: &mut HashSet<Variable>);
}

impl<T: UsedBinders> UsedBinders for Vec<T> {
    fn used_binders(&self, used: &mut HashSet<Variable>) {
        for element in self {
            element.used_binders(used);
        }
    }
}
