use super::{Statement, Var};

use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Substitute {
    pub rearrange: Vec<(Var, Var)>,
    pub next: Rc<Statement>,
}

impl std::fmt::Display for Substitute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rearrange = self
            .rearrange
            .iter()
            .map(|(new, old)| format!("({new} !-> {old})"))
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "substitute {};\n  {}", rearrange, self.next)
    }
}

impl From<Substitute> for Statement {
    fn from(value: Substitute) -> Self {
        Statement::Substitute(value)
    }
}
