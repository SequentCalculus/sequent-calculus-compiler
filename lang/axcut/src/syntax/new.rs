use super::{
    clause::Clause, context::TypingContext, names::Var, statement::Statement, stringify_and_join,
    types::Ty,
};

use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct New {
    pub var: Var,
    pub ty: Ty,
    pub env: TypingContext,
    pub clauses: Vec<Clause>,
    pub next: Rc<Statement>,
}

impl std::fmt::Display for New {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let env = stringify_and_join(&self.env, ", ");
        let clauses = stringify_and_join(&self.clauses, "\n    ");
        write!(
            f,
            "new {} : {} = ({}){{\n    {} }};\n  {}",
            self.var, self.ty, env, clauses, self.next
        )
    }
}

impl From<New> for Statement {
    fn from(value: New) -> Self {
        Statement::New(value)
    }
}
