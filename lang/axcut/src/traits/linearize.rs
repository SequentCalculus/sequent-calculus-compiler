use crate::syntax::Var;

use std::collections::HashSet;
use std::rc::Rc;

pub trait UsedBinders {
    fn used_binders(&self, used: &mut HashSet<Var>);
}

impl<T: UsedBinders> UsedBinders for Vec<T> {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        for element in self {
            element.used_binders(used);
        }
    }
}

#[must_use]
pub fn fresh_var(used_vars: &HashSet<Var>, base_name: &str) -> Var {
    fresh_var_n(used_vars, base_name, 0)
}

fn fresh_var_n(used_vars: &HashSet<Var>, base_name: &str, mut n: i32) -> Var {
    let mut new_var: Var = format!("{base_name}{n}");
    while used_vars.contains(&new_var) {
        n += 1;
        new_var = format!("{base_name}{n}");
    }
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
