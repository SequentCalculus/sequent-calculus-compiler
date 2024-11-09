use super::{names::filter_by_set, BinOp, Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use printer::tokens::{LEFT_ARROW, SEMI};
use printer::{DocAllocator, Print};

use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
    pub fst: Var,
    pub op: BinOp,
    pub snd: Var,
    pub var: Var,
    pub case: Rc<Statement>,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} <- {} {} {};\n  {}",
            self.var, self.fst, self.op, self.snd, self.case
        )
    }
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
            .append(alloc.space())
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

    fn subst_sim(self, subst: &[(Var, Var)]) -> Op {
        Op {
            fst: self.fst.subst_sim(subst),
            snd: self.snd.subst_sim(subst),
            case: self.case.subst_sim(subst),
            ..self
        }
    }
}

impl UsedBinders for Op {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        used.insert(self.var.clone());
        self.case.used_binders(used);
    }
}

impl Linearizing for Op {
    type Target = crate::syntax::Substitute;
    fn linearize(
        self,
        context: Vec<Var>,
        used_vars: &mut HashSet<Var>,
    ) -> crate::syntax::Substitute {
        let mut free_vars = HashSet::new();
        self.case.free_vars(&mut free_vars);
        free_vars.insert(self.fst.clone());
        free_vars.insert(self.snd.clone());

        let mut new_context = filter_by_set(&context, &free_vars);

        let rearrange = new_context
            .clone()
            .into_iter()
            .zip(new_context.clone())
            .collect();

        new_context.push(self.var.clone());

        crate::syntax::Substitute {
            rearrange,
            next: Rc::new(
                crate::syntax::Op {
                    fst: self.fst,
                    op: self.op,
                    snd: self.snd,
                    var: self.var,
                    case: self.case.linearize(new_context, used_vars),
                }
                .into(),
            ),
        }
    }
}
