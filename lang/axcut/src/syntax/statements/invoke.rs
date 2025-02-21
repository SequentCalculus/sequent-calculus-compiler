use printer::{theme::ThemeExt, tokens::INVOKE, DocAllocator, Print};

use super::Substitute;
use crate::syntax::{names::freshen, Name, Statement, Ty, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invoke {
    pub var: Var,
    pub tag: Name,
    pub ty: Ty,
    pub args: Vec<Var>,
}

impl Print for Invoke {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let args = if self.args.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };
        alloc
            .keyword(INVOKE)
            .append(alloc.space())
            .append(&self.var)
            .append(alloc.space())
            .append(&self.tag)
            .append(args)
    }
}

impl From<Invoke> for Statement {
    fn from(value: Invoke) -> Self {
        Statement::Invoke(value)
    }
}

impl FreeVars for Invoke {
    fn free_vars(self) -> (Self, HashSet<Var>) {
        let mut vars: HashSet<_> = self.args.iter().cloned().collect();
        vars.insert(self.var.clone());
        (self, vars)
    }
}

impl Subst for Invoke {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Invoke {
        self.var = self.var.subst_sim(subst);
        self.args = self.args.subst_sim(subst);
        self
    }
}

impl Linearizing for Invoke {
    type Target = Statement;
    fn linearize(mut self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let args = std::mem::take(&mut self.args);

        let mut context_rearrange = args.clone();
        context_rearrange.push(self.var.clone());

        if context == context_rearrange {
            self.into()
        } else {
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
