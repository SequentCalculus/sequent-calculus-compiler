use printer::tokens::{LEFT_ARROW, SEMI};
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{names::filter_by_set, BinOp, Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
    pub fst: Var,
    pub op: BinOp,
    pub snd: Var,
    pub var: Var,
    pub case: Rc<Statement>,
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
            .append(self.case.print(cfg, alloc))
    }
}

impl From<Op> for Statement {
    fn from(value: Op) -> Self {
        Statement::Op(value)
    }
}

impl FreeVars for Op {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.case.free_vars(vars);
        vars.remove(&self.var);
        vars.insert(self.fst.clone());
        vars.insert(self.snd.clone());
    }
}

impl Subst for Op {
    type Target = Op;

    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Op {
        self.fst = self.fst.subst_sim(subst);
        self.snd = self.snd.subst_sim(subst);

        self.case = self.case.subst_sim(subst);

        self
    }
}

impl Linearizing for Op {
    type Target = Statement;
    fn linearize(mut self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars = HashSet::new();
        self.case.free_vars(&mut free_vars);
        free_vars.insert(self.fst.clone());
        free_vars.insert(self.snd.clone());

        let mut new_context = filter_by_set(&context, &free_vars);
        let context_rearrange = new_context.clone();

        new_context.push(self.var.clone());
        self.case = self.case.linearize(new_context, used_vars);

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
