use super::{names::Name, statement::Statement, types::Ty};

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invoke {
    pub var: Name,
    pub tag: Name,
    pub ty: Ty,
}

impl std::fmt::Display for Invoke {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invoke {} {}", self.var, self.tag)
    }
}

impl From<Invoke> for Statement {
    fn from(value: Invoke) -> Self {
        Statement::Invoke(value)
    }
}
