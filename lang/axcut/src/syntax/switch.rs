use super::{stringify_and_join, Clause, Statement, Ty, Var};

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Switch {
    pub var: Var,
    pub ty: Ty,
    pub clauses: Vec<Clause>,
}

impl std::fmt::Display for Switch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clauses = stringify_and_join(&self.clauses, "\n    ");
        write!(f, "switch {} {{\n    {} }}", self.var, clauses)
    }
}

impl From<Switch> for Statement {
    fn from(value: Switch) -> Self {
        Statement::Switch(value)
    }
}
