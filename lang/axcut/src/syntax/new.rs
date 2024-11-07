use super::{stringify_and_join, Clause, Statement, Ty, Var};

use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct New {
    pub var: Var,
    pub ty: Ty,
    pub context: Vec<Var>,
    pub clauses: Vec<Clause>,
    pub next: Rc<Statement>,
}

impl std::fmt::Display for New {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let context = stringify_and_join(&self.context, ", ");
        let clauses = stringify_and_join(&self.clauses, "\n    ");
        write!(
            f,
            "new {} : {} = ({}){{\n    {} }};\n  {}",
            self.var, self.ty, context, clauses, self.next
        )
    }
}

impl From<New> for Statement {
    fn from(value: New) -> Self {
        Statement::New(value)
    }
}
