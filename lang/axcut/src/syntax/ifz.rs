use super::{names::Var, statement::Statement};
use std::{fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfZ {
    pub ifc: Var,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl std::fmt::Display for IfZ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ifz {} {{\n    () =>\n  {}\n    () =>\n  {} }}",
            self.ifc, self.thenc, self.elsec
        )
    }
}

impl From<IfZ> for Statement {
    fn from(value: IfZ) -> Self {
        Statement::IfZ(value)
    }
}
