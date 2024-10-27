use super::{Name, Statement};

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub label: Name,
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "jump {}", self.label)
    }
}

impl From<Call> for Statement {
    fn from(value: Call) -> Self {
        Statement::Call(value)
    }
}
