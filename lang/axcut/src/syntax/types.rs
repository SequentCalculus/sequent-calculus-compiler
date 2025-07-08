//! This module defines types in AxCut.

use printer::{Print, theme::ThemeExt, tokens::I64};

use super::{Name, TypeDeclaration};

/// This enum encodes the types of AxCut. They are either integers of names of user-declared types.
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum Ty {
    /// Signed 64-bit integer.
    I64,
    /// User-declared (data or codata) type.
    Decl(Name),
}

impl Ty {
    /// This function returns a reference to the declaration of the type name in a given list of
    /// type declarations.
    /// - `types` is the list of type declarations.
    ///
    /// # Panics
    /// A panic is caused if the type name is not in the list of type declarations or if the type
    /// is not a name of a user-declared type.
    pub fn lookup_type_declaration<'a>(&self, types: &'a [TypeDeclaration]) -> &'a TypeDeclaration {
        if let Ty::Decl(type_name) = self {
            types
                .iter()
                .find(|declaration| declaration.name == *type_name)
                .unwrap_or_else(|| panic!("Type {type_name} not found"))
        } else {
            panic!("User-defined type cannot be {}", self.print_to_string(None));
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
            Ty::I64 => alloc.typ(I64),
            Ty::Decl(name) => alloc.typ(name),
        }
    }
}
