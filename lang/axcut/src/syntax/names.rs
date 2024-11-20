use printer::tokens::{DIVIDE, MINUS, MODULO, PLUS, TIMES};
use printer::{DocAllocator, Print};

use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::fresh_var;
use crate::traits::substitution::Subst;

use std::collections::HashSet;

pub type Name = String;
pub type Var = String;

impl FreeVars for Var {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        vars.insert(self.clone());
    }
}

impl Subst for Var {
    type Target = Var;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Var {
        match subst.iter().find(|(old, _)| *old == self) {
            None => self,
            Some((_, new)) => new.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Div,
    Prod,
    Rem,
    Sum,
    Sub,
}

impl Print for BinOp {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            BinOp::Div => alloc.text(DIVIDE),
            BinOp::Prod => alloc.text(TIMES),
            BinOp::Rem => alloc.text(MODULO),
            BinOp::Sum => alloc.text(PLUS),
            BinOp::Sub => alloc.text(MINUS),
        }
    }
}

/// Picks fresh names for variables that are duplicated.
pub fn freshen(
    context: &[Var],
    mut clashes: HashSet<Var>,
    used_vars: &mut HashSet<Var>,
) -> Vec<Var> {
    let mut new_context = Vec::with_capacity(context.len());
    for var in context {
        if clashes.contains(var) {
            new_context.push(fresh_var(used_vars, var));
        } else {
            clashes.insert(var.clone());
            new_context.push(var.clone());
        }
    }
    new_context
}

/// Only keeps the binding in `context` which are contained in `set`, but tries to retain the
/// positions of as many bindings as possible.
#[must_use]
pub fn filter_by_set(context: &[Var], set: &HashSet<Var>) -> Vec<Var> {
    let mut new_context = context.to_owned();
    for (pos, var) in context.iter().enumerate() {
        if pos >= new_context.len() {
            break;
        } else if !set.contains(var) {
            let mut found_element = false;
            while new_context.len() - 1 > pos {
                if set.contains(&new_context[new_context.len() - 1]) {
                    found_element = true;
                    new_context.swap_remove(pos);
                    break;
                }
                new_context.pop();
            }
            if !found_element {
                new_context.pop();
            }
        }
    }
    new_context
}

#[cfg(test)]
mod names_test {
    use super::{filter_by_set, freshen, BinOp, FreeVars, Subst};
    use crate::syntax::Var;
    use printer::Print;
    use std::collections::HashSet;

    #[test]
    fn free_vars_var() {
        let mut result = HashSet::new();
        "x".to_owned().free_vars(&mut result);
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_var() {
        let result = "x"
            .to_owned()
            .subst_sim(&vec![("x".to_owned(), "y".to_owned())]);
        let expected = "y";
        assert_eq!(result, expected)
    }

    #[test]
    fn print_prod() {
        let result = BinOp::Prod.print_to_string(Default::default());
        let expected = "*";
        assert_eq!(result, expected)
    }

    #[test]
    fn print_sub() {
        let result = BinOp::Sub.print_to_string(Default::default());
        let expected = "-";
        assert_eq!(result, expected)
    }

    #[test]
    fn print_sum() {
        let result = BinOp::Sum.print_to_string(Default::default());
        let expected = "+";
        assert_eq!(result, expected)
    }

    #[test]
    fn freshen_vars_same() {
        let result = freshen(
            &vec!["x".to_owned(), "x".to_owned()],
            HashSet::new(),
            &mut HashSet::new(),
        );
        let expected = vec!["x".to_owned(), "x0".to_owned()];
        assert_eq!(result, expected)
    }

    #[test]
    fn freshen_vars_different() {
        let result = freshen(
            &vec!["x".to_owned(), "y".to_owned()],
            HashSet::new(),
            &mut HashSet::new(),
        );
        let expected = vec!["x".to_owned(), "y".to_owned()];
        assert_eq!(result, expected)
    }

    #[test]
    fn filter_context_empty() {
        let result = filter_by_set(
            &vec!["x".to_owned(), "y".to_owned(), "z".to_owned()],
            &HashSet::new(),
        );
        let expected: Vec<Var> = vec![];
        assert_eq!(result, expected)
    }

    #[test]
    fn filter_context() {
        let result = filter_by_set(
            &vec!["x".to_owned(), "y".to_owned(), "z".to_owned()],
            &HashSet::from(["x".to_owned(), "z".to_owned()]),
        );
        let expected = vec!["x".to_owned(), "z".to_owned()];
        assert_eq!(result, expected)
    }
}
