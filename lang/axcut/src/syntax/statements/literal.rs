use printer::theme::ThemeExt;
use printer::tokens::{LEFT_ARROW, LIT, SEMI};
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{names::filter_by_set, Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub lit: i64,
    pub var: Var,
    pub next: Rc<Statement>,
}

impl Print for Literal {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(LIT)
            .append(alloc.space())
            .append(&self.var)
            .append(alloc.space())
            .append(LEFT_ARROW)
            .append(alloc.space())
            .append(format!("{}", self.lit))
            .append(SEMI)
            .append(alloc.line())
            .append(self.next.print(cfg, alloc))
    }
}

impl From<Literal> for Statement {
    fn from(value: Literal) -> Self {
        Statement::Literal(value)
    }
}

impl FreeVars for Literal {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.next.free_vars(vars);
        vars.remove(&self.var);
    }
}

impl Subst for Literal {
    type Target = Literal;

    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Literal {
        self.next = self.next.subst_sim(subst);
        self
    }
}

impl Linearizing for Literal {
    type Target = Statement;
    fn linearize(mut self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars = HashSet::new();
        self.next.free_vars(&mut free_vars);

        let mut new_context = filter_by_set(&context, &free_vars);
        let context_rearrange = new_context.clone();

        new_context.push(self.var.clone());
        self.next = self.next.linearize(new_context, used_vars);

        if context == context_rearrange {
            self.into()
        } else {
            let rearrange = context_rearrange
                .clone()
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
