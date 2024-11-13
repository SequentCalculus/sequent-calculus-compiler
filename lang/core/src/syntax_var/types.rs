use super::Name;

use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ty {
    Int,
    Decl(Name),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ty::Int => f.write_str("Int"),
            Ty::Decl(name) => f.write_str(name),
        }
    }
}
