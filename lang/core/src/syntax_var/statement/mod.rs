use printer::{tokens::DONE, DocAllocator, Print};

use crate::syntax_var::Var;
use crate::traits::{substitution::SubstVar, used_binders::UsedBinders};

use std::collections::HashSet;

pub mod call;
pub mod cut;
pub mod ife;
pub mod ifl;
pub mod ifz;
pub mod op;

pub use call::*;
pub use cut::*;
pub use ife::*;
pub use ifl::*;
pub use ifz::*;
pub use op::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Cut(Cut),
    Op(Op),
    IfE(IfE),
    IfL(IfL),
    IfZ(IfZ),
    Call(Call),
    Done(),
}

impl Print for Statement {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Statement::Cut(cut) => cut.print(cfg, alloc),
            Statement::Op(op) => op.print(cfg, alloc),
            Statement::IfE(ife) => ife.print(cfg, alloc),
            Statement::IfL(ifl) => ifl.print(cfg, alloc),
            Statement::IfZ(ifz) => ifz.print(cfg, alloc),
            Statement::Call(call) => call.print(cfg, alloc),
            Statement::Done() => alloc.text(DONE),
        }
    }
}

impl UsedBinders for Statement {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        match self {
            Statement::Cut(cut) => cut.used_binders(used),
            Statement::Op(op) => op.used_binders(used),
            Statement::IfE(ife) => ife.used_binders(used),
            Statement::IfL(ifl) => ifl.used_binders(used),
            Statement::IfZ(ifz) => ifz.used_binders(used),
            _ => {}
        }
    }
}

impl SubstVar for Statement {
    type Target = Statement;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Statement {
        match self {
            Statement::Cut(cut) => cut.subst_sim(subst).into(),
            Statement::Op(op) => op.subst_sim(subst).into(),
            Statement::IfE(ife) => ife.subst_sim(subst).into(),
            Statement::IfL(ifl) => ifl.subst_sim(subst).into(),
            Statement::IfZ(ifz) => ifz.subst_sim(subst).into(),
            Statement::Call(call) => call.subst_sim(subst).into(),
            Statement::Done() => Statement::Done(),
        }
    }
}
