use super::{BinOp, Statement, Var};

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

impl From<Op> for Statement {
    fn from(value: Op) -> Self {
        Statement::Op(value)
    }
}
