use printer::{tokens::DONE, DocAllocator, Print};

use crate::syntax_var::Var;
use crate::traits::substitution::SubstVar;

pub mod call;
pub mod cut;
pub mod ifz;
pub mod op;

pub use call::*;
pub use cut::*;
pub use ifz::*;
pub use op::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Cut(Cut),
    Op(Op),
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
            Statement::IfZ(if_z) => if_z.print(cfg, alloc),
            Statement::Call(call) => call.print(cfg, alloc),
            Statement::Done() => alloc.text(DONE),
        }
    }
}

impl SubstVar for Statement {
    type Target = Statement;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Statement {
        match self {
            Statement::Cut(cut) => cut.subst_sim(subst).into(),
            Statement::Op(op) => op.subst_sim(subst).into(),
            Statement::IfZ(if_z) => if_z.subst_sim(subst).into(),
            Statement::Call(call) => call.subst_sim(subst).into(),
            Statement::Done() => Statement::Done(),
        }
    }
}
