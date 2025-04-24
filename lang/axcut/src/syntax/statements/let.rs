use printer::theme::ThemeExt;
use printer::tokens::{COLON, EQ, LET, SEMI};
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{
    Name, Statement, Ty, Var,
    names::{filter_by_set, freshen},
};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Let {
    pub var: Var,
    pub ty: Ty,
    pub tag: Name,
    pub args: Vec<Var>,
    pub next: Rc<Statement>,
    pub free_vars_next: Option<HashSet<Var>>,
}

impl Print for Let {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(LET)
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

impl From<Let> for Statement {
    fn from(value: Let) -> Self {
        Statement::Let(value)
    }
}

impl FreeVars for Let {
    fn free_vars(mut self, vars: &mut HashSet<Var>) -> Self {
        self.next = self.next.free_vars(vars);
        self.free_vars_next = Some(vars.clone());

        vars.remove(&self.var);
        vars.extend(self.args.iter().cloned());

        self
    }
}

impl Subst for Let {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Let {
        self.args = self.args.subst_sim(subst);
        self.next = self.next.subst_sim(subst);
        self.free_vars_next = self.free_vars_next.subst_sim(subst);
        self
    }
}

impl Linearizing for Let {
    type Target = Statement;
    fn linearize(mut self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let free_vars = std::mem::take(&mut self.free_vars_next)
            .expect("Free variables must be annotated before linearization");

        let mut new_context = filter_by_set(&context, &free_vars);

        let mut context_rearrange = new_context.clone();
        context_rearrange.append(&mut self.args.clone());

        if context == context_rearrange {
            new_context.push(self.var.clone());
            self.next = self.next.linearize(new_context, used_vars);
            self.into()
        } else {
            self.args = freshen(
                &self.args,
                new_context.clone().into_iter().collect(),
                used_vars,
            );

            let mut context_rearrange_freshened = new_context.clone();
            context_rearrange_freshened.append(&mut self.args.clone());

            new_context.push(self.var.clone());
            self.next = self.next.linearize(new_context, used_vars);

            let rearrange = context_rearrange_freshened
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
