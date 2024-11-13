use crate::syntax_var::Var;
use crate::traits::substitution::SubstVar;

use std::fmt;

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

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Cut(c) => c.fmt(f),
            Statement::Op(op) => op.fmt(f),
            Statement::IfZ(i) => i.fmt(f),
            Statement::Call(fun) => fun.fmt(f),
            Statement::Done() => write!(f, "Done"),
        }
    }
}

impl SubstVar for Statement {
    type Target = Statement;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Statement {
        match self {
            Statement::Cut(c) => c.subst_sim(subst).into(),
            Statement::Op(o) => o.subst_sim(subst).into(),
            Statement::IfZ(i) => i.subst_sim(subst).into(),
            Statement::Call(f) => f.subst_sim(subst).into(),
            Statement::Done() => Statement::Done(),
        }
    }
}
