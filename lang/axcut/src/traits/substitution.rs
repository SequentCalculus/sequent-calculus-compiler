use std::rc::Rc;

use crate::syntax::Var;

/// As all variable bindings are assumed to be unique, no care is needed to avoid captures or
/// shadowing.
pub trait Subst: Clone {
    type Target;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target;
}

impl<T: Subst> Subst for Rc<T> {
    type Target = Rc<T::Target>;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).subst_sim(subst))
    }
}

impl<T: Subst> Subst for Vec<T> {
    type Target = Vec<T::Target>;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Vec<T::Target> {
        self.into_iter()
            .map(|element| element.subst_sim(subst))
            .collect()
    }
}
