use printer::tokens::COLON;
use printer::{DocAllocator, Print};

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

impl Print for ContextBinding {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(&self.var)
            .append(alloc.space())
            .append(COLON)
            .append(self.chi.print(cfg, alloc))
            .append(COLON)
            .append(alloc.space())
            .append(self.ty.print(cfg, alloc))
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
