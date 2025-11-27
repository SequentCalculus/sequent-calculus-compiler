//! This module defines the call of a top-level function in AxCut.

use printer::Print;

use super::Substitute;
use crate::syntax::{ContextBinding, Name, Statement, TypingContext, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;
use crate::traits::typed_free_vars::TypedFreeVars;

use std::{
    collections::{BTreeSet, HashSet},
    rc::Rc,
};

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

impl TypedFreeVars for Call {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        vars.extend(self.args.bindings.iter().cloned());
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
    fn linearize(mut self, context: TypingContext, used_vars: &mut HashSet<Var>) -> Statement {
        let args = std::mem::take(&mut self.args.bindings).into();

        // the context must consist of the arguments for the top-level function
        if context == args {
            // if the context is exactly right already, we do not have to do anything
            self.into()
        } else {
            // otherwise we pick fresh names for duplicated variables via an explicit substitution
            let freshened_context = args.freshen(HashSet::new(), used_vars);
            let rearrange = freshened_context
                .bindings
                .into_iter()
                .zip(args.into_iter_vars())
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(self.into()),
            }
            .into()
        }
    }
}
