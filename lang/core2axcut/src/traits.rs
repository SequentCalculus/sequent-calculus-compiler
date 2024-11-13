use core::syntax_var::{TypeDeclaration, Var};

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
pub trait Shrinking {
    type Target;
    fn shrink(self, used_vars: &mut HashSet<Var>, types: &[TypeDeclaration]) -> Self::Target;
}

impl<T: Shrinking + Clone> Shrinking for Rc<T> {
    type Target = Rc<T::Target>;
    fn shrink(self, used_vars: &mut HashSet<Var>, types: &[TypeDeclaration]) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).shrink(used_vars, types))
    }
}

impl<T: Shrinking> Shrinking for Vec<T> {
    type Target = Vec<T::Target>;
    fn shrink(self, used_vars: &mut HashSet<Var>, types: &[TypeDeclaration]) -> Self::Target {
        self.into_iter()
            .map(|element| element.shrink(used_vars, types))
            .collect()
    }
}
