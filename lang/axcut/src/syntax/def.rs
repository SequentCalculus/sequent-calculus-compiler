use super::{context::TypingContext, names::Name, statement::Statement, stringify_and_join};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Def {
    pub name: Name,
    pub context: TypingContext,
    pub body: Statement,
}

impl std::fmt::Display for Def {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = stringify_and_join(&self.context, ", ");
        write!(f, "def {}({}) :=\n  {}", self.name, args, self.body)
    }
}
