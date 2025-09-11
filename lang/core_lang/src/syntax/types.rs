//! This module defines types in Core.

use printer::tokens::I64;
use printer::*;

use crate::syntax::*;

/// This enum encodes the types of AxCut. They are either integers or names of user-declared types.
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum Ty {
    /// Signed 64-Bit integer.
    I64,
    /// User-declared data or codata type.
    Decl(Name),
}

impl Ty {
    /// This function checks whether a type is a codata type.
    /// - `codata_types` is the list of codata type declarations in the program.
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
    fn print<'a>(&'a self, _cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        match self {
            Ty::I64 => alloc.typ(I64),
            Ty::Decl(name) => alloc.typ(name),
        }
    }
}
