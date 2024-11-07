use super::{stringify_and_join, Name, Statement, TypingContext};

use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub xtor: Name,
    pub context: TypingContext,
    pub case: Rc<Statement>,
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let context = stringify_and_join(&self.context, ", ");
        write!(f, "{}({context}) =>\n  {}", self.xtor, self.case)
    }
}
