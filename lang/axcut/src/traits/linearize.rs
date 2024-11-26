use crate::syntax::Var;

use std::collections::HashSet;
use std::rc::Rc;

#[must_use]
pub fn fresh_var(used_vars: &mut HashSet<Var>, base_name: &str) -> Var {
    let mut n = 0;
    let mut new_var: Var = format!("{base_name}{n}");
    while used_vars.contains(&new_var) {
        n += 1;
        new_var = format!("{base_name}{n}");
    }
    used_vars.insert(new_var.clone());
    new_var
}

/// This assumes all variable bindings to be unique and maintains this invariant.
pub trait Linearizing {
    type Target;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Self::Target;
}

impl<T: Linearizing + Clone> Linearizing for Rc<T> {
    type Target = Rc<T::Target>;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).linearize(context, used_vars))
    }
}
