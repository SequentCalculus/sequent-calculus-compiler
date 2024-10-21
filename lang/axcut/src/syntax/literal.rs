use super::{names::Var, statement::Statement};
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub lit: i64,
    pub var: Var,
    pub case: Rc<Statement>,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lit {} <- {};\n  {}", self.var, self.lit, self.case)
    }
}

impl From<Literal> for Statement {
    fn from(value: Literal) -> Self {
        Statement::Literal(value)
    }
}
