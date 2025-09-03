//! This module defines the call of a top-level function in AxCut.

use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{Name, Statement, Var, names::freshen};
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
    pub args: Vec<Var>,
}

impl Print for Call {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let sep = if cfg.allow_linebreaks {
            alloc.line_()
        } else {
            alloc.nil()
        };

        let args = if self.args.is_empty() {
            alloc.nil()
        } else {
            sep.clone()
                .append(self.args.print(cfg, alloc))
                .nest(cfg.indent)
                .append(sep)
        };

        self.label.print(cfg, alloc).append(args.parens().group())
    }
}

impl From<Call> for Statement {
    fn from(value: Call) -> Self {
        Statement::Call(value)
    }
}

impl FreeVars for Call {
    fn free_vars(self, vars: &mut HashSet<Var>) -> Self {
        vars.extend(self.args.iter().cloned());
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
    fn linearize(mut self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let args = std::mem::take(&mut self.args);

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
