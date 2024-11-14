use printer::{theme::ThemeExt, tokens::INT, Print};

use super::Name;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ty {
    Int,
    Decl(Name),
}

impl Print for Ty {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Ty::Int => alloc.keyword(INT),
            Ty::Decl(name) => alloc.typ(name),
        }
    }
}
