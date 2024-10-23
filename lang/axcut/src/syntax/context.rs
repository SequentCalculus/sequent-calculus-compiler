use super::{names::Var, polarity::Polarity, types::Ty};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::fresh_var;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct ContextBinding {
    pub var: Var,
    pub pol: Polarity,
    pub ty: Ty,
}

pub type TypingContext = Vec<ContextBinding>;

impl fmt::Display for ContextBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} :{}: {}", self.var, self.pol, self.ty)
    }
}

impl FreeVars for ContextBinding {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        vars.insert(self.var.clone());
    }
}

impl Subst for ContextBinding {
    type Target = ContextBinding;

    fn subst_sim(self, subst: &[(Var, Var)]) -> ContextBinding {
        ContextBinding {
            var: self.var.subst_sim(subst),
            ..self
        }
    }
}

/// Picks fresh names for all variables, which could be avoided by also passing the context with
/// which the variables that are not allowed to clash.
pub fn freshen(context: TypingContext, used_vars: &mut HashSet<Var>) -> TypingContext {
    let mut new_context = Vec::with_capacity(context.len());
    for binding in context {
        let new_var = fresh_var(used_vars, &binding.var);
        used_vars.insert(new_var.clone());
        new_context.push(ContextBinding {
            var: new_var,
            ..binding
        });
    }
    new_context
}

/// Only keeps the binding in `context` which are contained in `set`, but tries to retain the
/// positions of as many bindings as possible.
#[must_use]
pub fn filter_by_set(context: &TypingContext, set: &HashSet<Var>) -> TypingContext {
    let mut new_context = context.clone();
    for (pos, binding) in context.iter().enumerate() {
        if pos >= new_context.len() {
            break;
        } else if !set.contains(&binding.var) {
            let mut found_element = false;
            while new_context.len() - 1 > pos {
                if set.contains(&new_context[new_context.len() - 1].var) {
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

#[must_use]
pub fn context_vars(context: &TypingContext) -> Vec<Var> {
    let mut vars = Vec::with_capacity(context.len());
    for binding in context {
        vars.push(binding.var.clone());
    }
    vars
}
