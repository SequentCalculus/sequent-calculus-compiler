use printer::theme::ThemeExt;
use printer::tokens::{COLON, EQ, LETA, SEMI};
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{
    names::{filter_by_set, freshen},
    Name, Statement, Ty, Var,
};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Leta {
    pub var: Var,
    pub ty: Ty,
    pub tag: Name,
    pub args: Vec<Var>,
    pub next: Rc<Statement>,
}

impl Print for Leta {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(LETA)
            .append(alloc.space())
            .append(&self.var)
            .append(COLON)
            .append(alloc.space())
            .append(self.ty.print(cfg, alloc))
            .append(alloc.space())
            .append(EQ)
            .append(alloc.space())
            .append(&self.tag)
            .append(self.args.print(cfg, alloc).parens())
            .append(SEMI)
            .append(alloc.line())
            .append(self.next.print(cfg, alloc))
    }
}

impl From<Leta> for Statement {
    fn from(value: Leta) -> Self {
        Statement::Leta(value)
    }
}

impl FreeVars for Leta {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.next.free_vars(vars);
        vars.remove(&self.var);
        self.args.free_vars(vars);
    }
}

impl Subst for Leta {
    type Target = Leta;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Leta {
        Leta {
            args: self.args.subst_sim(subst),
            next: self.next.subst_sim(subst),
            ..self
        }
    }
}

impl UsedBinders for Leta {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        used.insert(self.var.clone());
        self.next.used_binders(used);
    }
}

impl Linearizing for Leta {
    type Target = Statement;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars = HashSet::new();
        self.next.free_vars(&mut free_vars);

        let mut new_context = filter_by_set(&context, &free_vars);

        let mut context_rearrange = new_context.clone();
        context_rearrange.append(&mut self.args.clone());

        if context == context_rearrange {
            new_context.push(self.var.clone());
            Leta {
                var: self.var,
                ty: self.ty,
                tag: self.tag,
                args: self.args,
                next: self.next.linearize(new_context, used_vars),
            }
            .into()
        } else {
            let freshened_context = freshen(
                &self.args,
                new_context.clone().into_iter().collect(),
                used_vars,
            );

            let mut context_rearrange_freshened = new_context.clone();
            context_rearrange_freshened.append(&mut freshened_context.clone());

            let rearrange = context_rearrange_freshened
                .into_iter()
                .zip(context_rearrange)
                .collect();
            new_context.push(self.var.clone());
            Substitute {
                rearrange,
                next: Rc::new(
                    Leta {
                        var: self.var,
                        ty: self.ty,
                        tag: self.tag,
                        args: freshened_context,
                        next: self.next.linearize(new_context, used_vars),
                    }
                    .into(),
                ),
            }
            .into()
        }
    }
}
