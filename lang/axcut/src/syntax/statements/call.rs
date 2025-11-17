//! This module defines the call of a top-level function in AxCut.

use printer::Print;

use super::Substitute;
use crate::syntax::{ContextBinding, Name, Statement, TypingContext, Var, names::freshen};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::{collections::HashSet, rc::Rc};

/// This struct defines the call of a top-level function in AxCut. It consists of the name of the
/// top-level function to call and the arguments. After linearization, the arguments are
/// immaterial, because the context then has to exactly fit the signature of the top-level
/// function anyway.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub label: Name,
    pub context: TypingContext,
}

impl Print for Call {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.label
            .print(cfg, alloc)
            .append(self.context.print(cfg, alloc).parens().group())
    }
}

impl From<Call> for Statement {
    fn from(value: Call) -> Self {
        Statement::Call(value)
    }
}

impl FreeVars for Call {
    fn free_vars(self, vars: &mut HashSet<Var>) -> Self {
        vars.extend(self.context.vars());
        self
    }
}

impl Subst for Call {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Call {
        let mut new_bindings = vec![];
        for binding in self.context.bindings {
            new_bindings.push(ContextBinding {
                var: binding.var.subst_sim(subst),
                ty: binding.ty,
                chi: binding.chi,
            });
        }
        self.context.bindings = new_bindings;
        self
    }
}

impl Linearizing for Call {
    type Target = Statement;
    fn linearize(self, context: TypingContext, used_vars: &mut HashSet<Var>) -> Statement {
        // the context must consist of the arguments for the top-level function
        if context == self.context {
            // if the context is exactly right already, we do not have to do anything
            self.into()
        } else {
            // otherwise we pick fresh names for duplicated variables via an explicit substitution
            let freshened_context = freshen(&self.context, HashSet::new(), used_vars);
            let rearrange = freshened_context
                .bindings
                .iter()
                .map(|bnd| &bnd.var)
                .cloned()
                .zip(self.context.bindings.iter().map(|bnd| &bnd.var).cloned())
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(self.into()),
            }
            .into()
        }
    }
}
