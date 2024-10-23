use super::names::Var;
use super::statement::Statement;
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

impl From<Return> for Statement {
    fn from(value: Return) -> Self {
        Statement::Return(value)
    }
}
