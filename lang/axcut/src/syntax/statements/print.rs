use printer::theme::ThemeExt;
use printer::tokens::{PRINTLN_I64, SEMI};
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{names::filter_by_set, Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintLnI64 {
    pub var: Var,
    pub next: Rc<Statement>,
}

impl Print for PrintLnI64 {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(PRINTLN_I64)
            .append(self.var.print(cfg, alloc).parens())
            .append(SEMI)
            .append(alloc.line())
            .append(self.next.print(cfg, alloc))
    }
}

impl From<PrintLnI64> for Statement {
    fn from(value: PrintLnI64) -> Self {
        Statement::PrintLnI64(value)
    }
}

impl FreeVars for PrintLnI64 {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.next.free_vars(vars);
        vars.insert(self.var.clone());
    }
}

impl Subst for PrintLnI64 {
    type Target = PrintLnI64;

    fn subst_sim(self, subst: &[(Var, Var)]) -> PrintLnI64 {
        PrintLnI64 {
            var: self.var.subst_sim(subst),
            next: self.next.subst_sim(subst),
        }
    }
}

impl Linearizing for PrintLnI64 {
    type Target = Statement;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars = HashSet::new();
        self.next.free_vars(&mut free_vars);
        free_vars.insert(self.var.clone());

        let new_context = filter_by_set(&context, &free_vars);
        let context_rearrange = new_context.clone();

        let print = PrintLnI64 {
            var: self.var,
            next: self.next.linearize(new_context, used_vars),
        }
        .into();

        if context == context_rearrange {
            print
        } else {
            let rearrange = context_rearrange
                .clone()
                .into_iter()
                .zip(context_rearrange.clone())
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(print),
            }
            .into()
        }
    }
}
