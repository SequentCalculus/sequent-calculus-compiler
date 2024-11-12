use super::{Chirality, Ty, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct ContextBinding {
    pub var: Var,
    pub chi: Chirality,
    pub ty: Ty,
}

pub type TypingContext = Vec<ContextBinding>;

impl fmt::Display for ContextBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} :{}: {}", self.var, self.chi, self.ty)
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

#[must_use]
pub fn context_vars(context: &TypingContext) -> Vec<Var> {
    let mut vars = Vec::with_capacity(context.len());
    for binding in context {
        vars.push(binding.var.clone());
    }
    vars
}

#[must_use]
pub fn lookup_variable_context<'a>(var: &str, context: &'a [ContextBinding]) -> &'a ContextBinding {
    let context_binding = context
        .iter()
        .find(|binding| var == binding.var)
        .expect("Variable {var} not found in context {context:?}");
    context_binding
}
