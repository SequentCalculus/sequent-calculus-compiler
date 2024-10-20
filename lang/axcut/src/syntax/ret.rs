use super::names::Var;
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
