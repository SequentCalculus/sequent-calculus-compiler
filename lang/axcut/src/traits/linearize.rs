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

#[cfg(test)]
mod linearize_tests {
    use super::fresh_var;
    use std::collections::HashSet;

    #[test]
    fn fresh_var_empty() {
        let result = fresh_var(&mut HashSet::new(), "x");
        let expected = "x0".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn fresh_var_vars() {
        let result = fresh_var(
            &mut HashSet::from(["x0".to_owned(), "x1".to_owned(), "x3".to_owned()]),
            "x",
        );
        let expected = "x2".to_owned();
        assert_eq!(result, expected)
    }
}
