use super::context::TypingContext;
use super::statement::Statement;
use super::stringify_and_join;

use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub env: TypingContext,
    pub case: Rc<Statement>,
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let env = stringify_and_join(&self.env, ", ");
        write!(f, "({}) =>\n  {}", env, self.case)
    }
}
