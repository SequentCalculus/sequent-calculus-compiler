use printer::tokens::{MINUS, PLUS, TIMES};
use printer::{DocAllocator, Print};

use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::fresh_var;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::fmt;

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
    Prod,
    Sum,
    Sub,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Prod => write!(f, "*"),
            BinOp::Sum => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
        }
    }
}

impl Print for BinOp {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            BinOp::Prod => alloc.text(TIMES),
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
