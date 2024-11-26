use printer::theme::ThemeExt;
use printer::{tokens::DONE, Print};

use crate::syntax::statement::{FsCall, FsIfE, FsIfL, FsIfZ, FsOp};
use crate::syntax_var::Var;
use crate::traits::substitution::SubstVar;

pub mod cut;

pub use cut::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsStatement {
    Cut(FsCut),
    Op(FsOp),
    IfE(FsIfE),
    IfL(FsIfL),
    IfZ(FsIfZ),
    Call(FsCall),
    Done(),
}

impl Print for FsStatement {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            FsStatement::Cut(cut) => cut.print(cfg, alloc),
            FsStatement::Op(op) => op.print(cfg, alloc),
            FsStatement::IfE(ife) => ife.print(cfg, alloc),
            FsStatement::IfL(ifl) => ifl.print(cfg, alloc),
            FsStatement::IfZ(ifz) => ifz.print(cfg, alloc),
            FsStatement::Call(call) => call.print(cfg, alloc),
            FsStatement::Done() => alloc.keyword(DONE),
        }
    }
}

impl SubstVar for FsStatement {
    type Target = FsStatement;
    fn subst_sim(self, subst: &[(Var, Var)]) -> FsStatement {
        match self {
            FsStatement::Cut(cut) => cut.subst_sim(subst).into(),
            FsStatement::Op(op) => op.subst_sim(subst).into(),
            FsStatement::IfE(ife) => ife.subst_sim(subst).into(),
            FsStatement::IfL(ifl) => ifl.subst_sim(subst).into(),
            FsStatement::IfZ(ifz) => ifz.subst_sim(subst).into(),
            FsStatement::Call(call) => call.subst_sim(subst).into(),
            FsStatement::Done() => FsStatement::Done(),
        }
    }
}
