use printer::{
    tokens::{COMMA, SEMI},
    DocAllocator, Print,
};

use crate::{
    syntax::BinOp,
    syntax_var::{term::FsTerm, FsStatement, Var},
    traits::substitution::SubstVar,
};

use std::rc::Rc;

/// Focused binary operation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsOp {
    pub fst: Var,
    pub op: BinOp,
    pub snd: Var,
    pub continuation: Rc<FsTerm>,
}

impl Print for FsOp {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.op.print(cfg, alloc).append(
            alloc
                .text(&self.fst)
                .append(alloc.text(COMMA))
                .append(alloc.space())
                .append(alloc.text(&self.snd))
                .append(SEMI)
                .append(alloc.space())
                .append(self.continuation.print(cfg, alloc))
                .parens(),
        )
    }
}

impl From<FsOp> for FsStatement {
    fn from(value: FsOp) -> Self {
        FsStatement::Op(value)
    }
}

impl SubstVar for FsOp {
    type Target = FsOp;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        FsOp {
            fst: self.fst.subst_sim(subst),
            op: self.op,
            snd: self.snd.subst_sim(subst),
            continuation: self.continuation.subst_sim(subst),
        }
    }
}
