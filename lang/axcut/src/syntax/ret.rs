use printer::Print;

use super::{Statement, Var};

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Return {
    pub var: Var,
}

impl std::fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return {}", self.var)
    }
}

impl Print for Return {
    fn print<'a>(&'a self, cfg: &printer::PrintCfg, alloc: &'a printer::Alloc<'a>) -> printer::Builder<'a> {
        todo!()
    }
}

impl From<Return> for Statement {
    fn from(value: Return) -> Self {
        Statement::Return(value)
    }
}
