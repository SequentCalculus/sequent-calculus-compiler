use super::{Call, IfZ, Invoke, Leta, Literal, New, Op, Return, Substitute, Switch};

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Substitute(Substitute),
    Call(Call),
    Leta(Leta),
    Switch(Switch),
    New(New),
    Invoke(Invoke),
    Literal(Literal),
    Op(Op),
    IfZ(IfZ),
    Return(Return),
    Done,
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Substitute(s) => s.fmt(f),
            Statement::Call(j) => j.fmt(f),
            Statement::Leta(l) => l.fmt(f),
            Statement::Switch(s) => s.fmt(f),
            Statement::New(n) => n.fmt(f),
            Statement::Invoke(i) => i.fmt(f),
            Statement::Literal(n) => n.fmt(f),
            Statement::Op(o) => o.fmt(f),
            Statement::IfZ(i) => i.fmt(f),
            Statement::Return(r) => r.fmt(f),
            Statement::Done => write!(f, "Done"),
        }
    }
}
