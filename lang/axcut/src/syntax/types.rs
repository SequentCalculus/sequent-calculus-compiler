use printer::Print;

use super::names::Name;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
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

impl Print for Ty {
    fn print<'a>(&'a self, cfg: &printer::PrintCfg, alloc: &'a printer::Alloc<'a>) -> printer::Builder<'a> {
        todo!()
    }
}
