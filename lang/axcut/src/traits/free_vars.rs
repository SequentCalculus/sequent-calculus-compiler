use crate::syntax::Var;

use std::collections::HashSet;
use std::rc::Rc;

/// Computing the free variables of a statement.
/// Assumes that the set `vars` passed to it is empty.
pub trait FreeVars: Sized {
    fn free_vars(self, vars: &mut HashSet<Var>) -> Self;
}

impl<T: FreeVars + Clone> FreeVars for Rc<T> {
    fn free_vars(self, vars: &mut HashSet<Var>) -> Self {
        Rc::new(Rc::unwrap_or_clone(self).free_vars(vars))
    }
}

impl<T: FreeVars> FreeVars for Vec<T> {
    fn free_vars(self, vars: &mut HashSet<Var>) -> Self {
        self.into_iter()
            .map(|element| {
                let mut free_vars = HashSet::new();
                let element = element.free_vars(&mut free_vars);
                vars.extend(free_vars);
                element
            })
            .collect()
    }
}
