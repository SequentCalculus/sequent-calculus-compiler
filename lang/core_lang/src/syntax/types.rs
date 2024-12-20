use printer::{theme::ThemeExt, tokens::I64, Print};

use super::{declaration::CodataDeclaration, Name};

/// Types
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ty {
    // Signed 64-Bit integer.
    I64,
    /// Declared data or codata type.
    Decl(Name),
}

impl Ty {
    #[must_use]
    pub fn is_codata(&self, codata_types: &[CodataDeclaration]) -> bool {
        match self {
            Ty::I64 => false,
            Ty::Decl(name) => codata_types
                .iter()
                .any(|declaration| declaration.name == *name),
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
            Ty::I64 => alloc.keyword(I64),
            Ty::Decl(name) => alloc.typ(name),
        }
    }
}
