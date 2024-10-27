use super::{stringify_and_join, Name, Statement, Ty, Var};

use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Leta {
    pub var: Var,
    pub ty: Ty,
    pub tag: Name,
    pub args: Vec<Var>,
    pub next: Rc<Statement>,
}

impl std::fmt::Display for Leta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = stringify_and_join(&self.args, ", ");
        write!(
            f,
            "leta {} : {} = {}({});\n  {}",
            self.var, self.ty, self.tag, args, self.next
        )
    }
}

impl From<Leta> for Statement {
    fn from(value: Leta) -> Self {
        Statement::Leta(value)
    }
}
