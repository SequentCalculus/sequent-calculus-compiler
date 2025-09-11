//! This module defines the invocation of a method of a closure in AxCut.

use printer::{DocAllocator, Print, theme::ThemeExt, tokens::INVOKE};

use super::Substitute;
use crate::syntax::{Arguments, Name, Statement, Ty, Var, names::freshen};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

/// This struct defines the invocation of a method of a closure in AxCut. It consists of the
/// variable the closure is bound to, the name of the method to invoke and its type, and the
/// arguments of the method. After linearization, the arguments are immaterial, because the context
/// then has to exactly fit the signature of the method anyway.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invoke {
    pub var: Var,
    pub tag: Name,
    pub ty: Ty,
    pub args: Arguments,
}

impl Print for Invoke {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let args = if self.args.entries.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };

        alloc
            .keyword(INVOKE)
            .append(alloc.space())
            .append(self.var.print(cfg, alloc))
            .append(alloc.space())
            .append(self.tag.print(cfg, alloc))
            .append(args.group())
    }
}

impl From<Invoke> for Statement {
    fn from(value: Invoke) -> Self {
        Statement::Invoke(value)
    }
}

impl FreeVars for Invoke {
    fn free_vars(self, vars: &mut HashSet<Var>) -> Self {
        vars.extend(self.args.entries.iter().cloned());
        vars.insert(self.var.clone());
        self
    }
}

impl Subst for Invoke {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Invoke {
        self.var = self.var.subst_sim(subst);
        self.args.entries = self.args.entries.subst_sim(subst);
        self
    }
}

impl Linearizing for Invoke {
    type Target = Statement;
    fn linearize(mut self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let args = std::mem::take(&mut self.args.entries);

        // the context must consist of the arguments for the method ...
        let mut context_rearrange = args.clone();
        // ... followed by the binding of the closure
        context_rearrange.push(self.var.clone());

        if context == context_rearrange {
            // if the context is exactly right already, we do not have to do anything
            self.into()
        } else {
            // otherwise we pick fresh names for duplicated variables via an explicit substitution
            let mut freshened_context = freshen(&args, HashSet::new(), used_vars);
            freshened_context.push(self.var.clone());

            let rearrange: Vec<(Var, Var)> = freshened_context
                .into_iter()
                .zip(context_rearrange)
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(self.into()),
            }
            .into()
        }
    }
}
