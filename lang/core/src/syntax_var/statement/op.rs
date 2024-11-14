use printer::{
    tokens::{COMMA, SEMI},
    DocAllocator, Print,
};

use crate::{
    syntax_var::{term::Term, BinOp, Statement, Var},
    traits::substitution::SubstVar,
};

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
    pub fst: Var,
    pub op: BinOp,
    pub snd: Var,
    pub continuation: Rc<Term>,
}

impl Print for Op {
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

impl From<Op> for Statement {
    fn from(value: Op) -> Self {
        Statement::Op(value)
    }
}

impl SubstVar for Op {
    type Target = Op;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        Op {
            fst: self.fst.subst_sim(subst),
            op: self.op,
            snd: self.snd.subst_sim(subst),
            continuation: self.continuation.subst_sim(subst),
        }
    }
}
