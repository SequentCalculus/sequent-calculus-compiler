use std::collections::HashSet;

use crate::syntax::Variable;

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
