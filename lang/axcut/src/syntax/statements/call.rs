//! This module defines the call of a top-level function in AxCut.

use printer::Print;

use super::Substitute;
use crate::syntax::{Name, Statement, TypingContext, Var};
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
    pub args: TypingContext,
}

impl Print for Call {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.label
            .print(cfg, alloc)
            .append(self.args.print(cfg, alloc).parens().group())
    }
}

impl From<Call> for Statement {
    fn from(value: Call) -> Self {
        Statement::Call(value)
    }
}

impl FreeVars for Call {
    fn free_vars(self, vars: &mut HashSet<Var>) -> Self {
        vars.extend(self.args.vars());
        self
    }
}

impl Subst for Call {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Call {
        self.args = self.args.subst_sim(subst);
        self
    }
}

impl Linearizing for Call {
    type Target = Statement;
    fn linearize(self, context: TypingContext, used_vars: &mut HashSet<Var>) -> Statement {
        // the context must consist of the arguments for the top-level function
        if context == self.args {
            // if the context is exactly right already, we do not have to do anything
            self.into()
        } else {
            // otherwise we pick fresh names for duplicated variables via an explicit substitution
            let freshened_context = self.args.freshen(HashSet::new(), used_vars);
            let rearrange = freshened_context
                .bindings
                .iter()
                .map(|bnd| &bnd.var)
                .cloned()
                .zip(self.args.bindings.iter().map(|bnd| &bnd.var).cloned())
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(self.into()),
            }
            .into()
        }
    }
}
