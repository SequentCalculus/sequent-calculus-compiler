use crate::syntax::Var;

use std::collections::HashSet;
use std::rc::Rc;

/// Computing the free variables of a statement.
pub trait FreeVars: Sized {
    fn free_vars(self) -> (Self, HashSet<Var>);
}

impl<T: FreeVars + Clone> FreeVars for Rc<T> {
    fn free_vars(self) -> (Self, HashSet<Var>) {
        let (t, vars) = Rc::unwrap_or_clone(self).free_vars();
        (Rc::new(t), vars)
    }
}

impl<T: FreeVars> FreeVars for Vec<T> {
    fn free_vars(self) -> (Self, HashSet<Var>) {
        let mut free_vars = HashSet::new();
        let mut elements = Vec::with_capacity(self.len());
        for element in self {
            let (element, vars) = element.free_vars();
            elements.push(element);
            free_vars.extend(vars);
        }
        (elements, free_vars)
    }
}
