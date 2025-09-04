//! This module defines the call of a top-level function in AxCut.

use printer::Print;

use super::Substitute;
use crate::syntax::{Arguments, Name, Statement, Var, names::freshen};
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
    pub args: Arguments,
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
        vars.extend(self.args.entries.iter().cloned());
        self
    }
}

impl Subst for Call {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Call {
        self.args.entries = self.args.entries.subst_sim(subst);
        self
    }
}

impl Linearizing for Call {
    type Target = Statement;
    fn linearize(mut self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let args = std::mem::take(&mut self.args.entries);

        // the context must consist of the arguments for the top-level function
        if context == args {
            // if the context is exactly right already, we do not have to do anything
            self.into()
        } else {
            // otherwise we pick fresh names for duplicated variables via an explicit substitution
            let freshened_context = freshen(&args, HashSet::new(), used_vars);
            let rearrange = freshened_context.into_iter().zip(args).collect();
            Substitute {
                rearrange,
                next: Rc::new(self.into()),
            }
            .into()
        }
    }
}
