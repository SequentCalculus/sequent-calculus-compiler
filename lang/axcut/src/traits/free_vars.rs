use std::collections::HashSet;

use crate::syntax::Var;

/// Computing the free variables of a statement.
pub trait FreeVars {
    fn free_vars(&self, vars: &mut HashSet<Var>);
}

impl<T: FreeVars> FreeVars for Vec<T> {
    fn free_vars(self: &Vec<T>, vars: &mut HashSet<Var>) {
        for element in self {
            element.free_vars(vars);
        }
    }
}
