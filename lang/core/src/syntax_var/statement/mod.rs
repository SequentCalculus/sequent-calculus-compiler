use printer::theme::ThemeExt;
use printer::{tokens::DONE, Print};

use crate::syntax_var::Var;
use crate::traits::substitution::SubstVar;

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
            Statement::Done() => alloc.keyword(DONE),
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
