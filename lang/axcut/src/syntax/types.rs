use printer::{theme::ThemeExt, tokens::INT, Print};

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
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Ty::Int => alloc.typ(INT),
            Ty::Decl(name) => alloc.typ(name),
        }
    }
}
