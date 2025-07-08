//! This module defines printing an integer in AxCut.

use printer::theme::ThemeExt;
use printer::tokens::{PRINT_I64, PRINTLN_I64, SEMI};
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{Statement, Var, names::filter_by_set};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

/// This struct defines printing an integer in AxCut. It consists of the information whether a
/// newline should be printed, the variable for the integer to print, and the remaining statement.
/// Moreover, the free variables of the remaining statement can be annotated.
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
    /// # Panics
    ///
    /// In this implementation of [`Linearizing::linearize`] a panic is caused if the free
    /// variables of the clauses and the remaining statement of a closure are not annotated.
    fn linearize(mut self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars = std::mem::take(&mut self.free_vars_next)
            .expect("Free variables must be annotated before linearization");
        // the variables is not consumed, so we have to keep it
        free_vars.insert(self.var.clone());

        // the new context consists of the context for the remaining statement
        let new_context = filter_by_set(&context, &free_vars);
        let context_rearrange = new_context.clone();

        // linearize the remaining statement
        self.next = self.next.linearize(new_context, used_vars);

        if context == context_rearrange {
            // if the context is exactly right already, we do not have to do anything
            self.into()
        } else {
            // otherwise we insert an explicit substitution
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
