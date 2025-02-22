use printer::theme::ThemeExt;
use printer::tokens::{PRINTLN_I64, PRINT_I64, SEMI};
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{names::filter_by_set, Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintI64 {
    pub newline: bool,
    pub var: Var,
    pub next: Rc<Statement>,
    pub free_vars_next: Option<HashSet<Var>>,
}

impl Print for PrintI64 {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let print_i64 = if self.newline { PRINTLN_I64 } else { PRINT_I64 };
        alloc
            .keyword(print_i64)
            .append(self.var.print(cfg, alloc).parens())
            .append(SEMI)
            .append(alloc.line())
            .append(self.next.print(cfg, alloc))
    }
}

impl From<PrintI64> for Statement {
    fn from(value: PrintI64) -> Self {
        Statement::PrintI64(value)
    }
}

impl FreeVars for PrintI64 {
    fn free_vars(mut self, vars: &mut HashSet<Var>) -> Self {
        self.next = self.next.free_vars(vars);
        self.free_vars_next = Some(vars.clone());

        vars.insert(self.var.clone());

        self
    }
}

impl Subst for PrintI64 {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> PrintI64 {
        self.var = self.var.subst_sim(subst);
        self.next = self.next.subst_sim(subst);
        self.free_vars_next = self.free_vars_next.subst_sim(subst);
        self
    }
}

impl Linearizing for PrintI64 {
    type Target = Statement;
    fn linearize(mut self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars = std::mem::take(&mut self.free_vars_next)
            .expect("Free variables must be annotated before linearization");
        free_vars.insert(self.var.clone());

        let new_context = filter_by_set(&context, &free_vars);
        let context_rearrange = new_context.clone();

        self.next = self.next.linearize(new_context, used_vars);

        if context == context_rearrange {
            self.into()
        } else {
            let rearrange = context_rearrange
                .clone()
                .into_iter()
                .zip(context_rearrange.clone())
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(self.into()),
            }
            .into()
        }
    }
}
