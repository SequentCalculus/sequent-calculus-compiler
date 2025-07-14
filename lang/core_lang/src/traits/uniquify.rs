use crate::syntax::Var;
use std::collections::HashSet;
use std::rc::Rc;

pub trait Uniquify {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Self;
}

impl<T: Uniquify + Clone> Uniquify for Rc<T> {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Self {
        Rc::new(Rc::unwrap_or_clone(self).uniquify(seen_vars, used_vars))
    }
}

impl<T: Uniquify> Uniquify for Option<T> {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Self {
        self.map(|t| t.uniquify(seen_vars, used_vars))
    }
}

impl<T: Uniquify> Uniquify for Vec<T> {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Self {
        self.into_iter()
            .map(|element| element.uniquify(seen_vars, used_vars))
            .collect()
    }
}
