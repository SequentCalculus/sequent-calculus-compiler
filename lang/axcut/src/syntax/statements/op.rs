//! This module defines arithmetic binary operations in AxCut.

use printer::tokens::{DIVIDE, LEFT_ARROW, MINUS, MODULO, PLUS, SEMI, TIMES};
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{Statement, Var, names::filter_by_set};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

/// This enum encodes the different kinds of arithmetic binary operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    /// Division
    Div,
    /// Multiplication
    Prod,
    /// Remainder
    Rem,
    /// Addition
    Sum,
    /// Subtraction
    Sub,
}

impl Print for BinOp {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            BinOp::Div => alloc.text(DIVIDE),
            BinOp::Prod => alloc.text(TIMES),
            BinOp::Rem => alloc.text(MODULO),
            BinOp::Sum => alloc.text(PLUS),
            BinOp::Sub => alloc.text(MINUS),
        }
    }
}

/// This struct defines arithmetic binary operations in AxCut. They consist of the input variables,
/// the kind of the binary operator, the variable the result is bound to, and the remaining
/// statement. Moreover, the free variables of the remaining statement can be annotated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
    pub fst: Var,
    pub op: BinOp,
    pub snd: Var,
    pub var: Var,
    pub next: Rc<Statement>,
    pub free_vars_next: Option<HashSet<Var>>,
}

impl Print for Op {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(&self.var)
            .append(alloc.space())
            .append(LEFT_ARROW)
            .append(alloc.space())
            .append(self.fst.print(cfg, alloc))
            .append(alloc.space())
            .append(self.op.print(cfg, alloc))
            .append(alloc.space())
            .append(self.snd.print(cfg, alloc))
            .append(SEMI)
            .append(alloc.line())
            .append(self.next.print(cfg, alloc))
    }
}

impl From<Op> for Statement {
    fn from(value: Op) -> Self {
        Statement::Op(value)
    }
}

impl FreeVars for Op {
    fn free_vars(mut self, vars: &mut HashSet<Var>) -> Self {
        self.next = self.next.free_vars(vars);
        self.free_vars_next = Some(vars.clone());

        vars.remove(&self.var);
        vars.insert(self.fst.clone());
        vars.insert(self.snd.clone());

        self
    }
}

impl Subst for Op {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Op {
        self.fst = self.fst.subst_sim(subst);
        self.snd = self.snd.subst_sim(subst);

        self.next = self.next.subst_sim(subst);
        self.free_vars_next = self.free_vars_next.subst_sim(subst);

        self
    }
}

impl Linearizing for Op {
    type Target = Statement;
    /// # Panics
    ///
    /// In this implementation of [`Linearizing::linearize`] a panic is caused if the free
    /// variables of the remaining statement are not annotated.
    fn linearize(mut self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars = std::mem::take(&mut self.free_vars_next)
            .expect("Free variables must be annotated before linearization");
        // the input variables are not consumed, so we have to keep them
        free_vars.insert(self.fst.clone());
        free_vars.insert(self.snd.clone());

        // the new context consists of the context for the remaining statement ...
        let mut new_context = filter_by_set(&context, &free_vars);
        let context_rearrange = new_context.clone();
        // ... and the variable the result is bound to
        new_context.push(self.var.clone());

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
