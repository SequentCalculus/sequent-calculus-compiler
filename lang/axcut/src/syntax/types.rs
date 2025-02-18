use printer::{theme::ThemeExt, tokens::I64, Print};

use super::{Name, TypeDeclaration};

/// Types
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum Ty {
    /// Signed 64-Bit integer.
    I64,
    /// Declared data or codata type.
    Decl(Name),
}

impl Ty {
    pub fn lookup_type_declaration<'a>(&self, types: &'a [TypeDeclaration]) -> &'a TypeDeclaration {
        if let Ty::Decl(type_name) = self {
            let type_declaration = types
                .iter()
                .find(|declaration| declaration.name == *type_name)
                .unwrap_or_else(|| panic!("Type {type_name} not found"));
            type_declaration
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
