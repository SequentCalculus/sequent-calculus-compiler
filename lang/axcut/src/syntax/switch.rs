use super::{clause::Clause, names::Var, statement::Statement, stringify_and_join};

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Switch {
    pub var: Var,
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
