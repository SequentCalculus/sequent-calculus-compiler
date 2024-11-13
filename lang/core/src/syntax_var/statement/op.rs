use crate::{
    syntax_var::{term::Term, BinOp, Statement, Var},
    traits::substitution::SubstVar,
};
use std::{fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
    pub fst: Var,
    pub op: BinOp,
    pub snd: Var,
    pub continuation: Rc<Term>,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({}, {}; {})",
            self.op, self.fst, self.snd, self.continuation
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
