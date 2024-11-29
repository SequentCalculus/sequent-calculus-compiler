use printer::{theme::ThemeExt, tokens::INT, Print};

use super::{Name, TypeDeclaration};

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
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
            Ty::Int => alloc.typ(INT),
            Ty::Decl(name) => alloc.typ(name),
        }
    }
}

impl Ty {
    #[must_use]
    pub fn lookup_type_declaration<'a>(&self, types: &'a [TypeDeclaration]) -> &'a TypeDeclaration {
        if let Ty::Decl(type_name) = self {
            let type_declaration = types
                .iter()
                .find(|declaration| declaration.name == *type_name)
                .expect("Type {type_name} not found");
            type_declaration
        } else {
            panic!("User-defined type cannot be {}", self.print_to_string(None));
        }
    }
}
