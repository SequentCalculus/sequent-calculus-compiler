use std::collections::HashSet;
use std::rc::Rc;

use crate::syntax::Var;

/// As all variable bindings are assumed to be unique, no care is needed to avoid captures or
/// shadowing.
pub trait Subst: Clone {
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self;
}

impl<T: Subst> Subst for Rc<T> {
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self {
        Rc::new(Rc::unwrap_or_clone(self).subst_sim(subst))
    }
}

impl<T: Subst> Subst for Vec<T> {
    fn subst_sim(self, subst: &[(Var, Var)]) -> Vec<T> {
        self.into_iter()
            .map(|element| element.subst_sim(subst))
            .collect()
    }
}

impl<T: Subst + std::hash::Hash + Eq> Subst for HashSet<T> {
    fn subst_sim(self, subst: &[(Var, Var)]) -> HashSet<T> {
        self.into_iter()
            .map(|element| element.subst_sim(subst))
            .collect()
    }
}

impl<T: Subst> Subst for Option<T> {
    fn subst_sim(self, subst: &[(Var, Var)]) -> Option<T> {
        self.map(|t| t.subst_sim(subst))
    }
}
