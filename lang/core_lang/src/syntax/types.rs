use printer::{theme::ThemeExt, tokens::INT, Print};

use super::{declaration::CodataDeclaration, Name};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ty {
    Int,
    Decl(Name),
}

impl Ty {
    #[must_use]
    pub fn is_codata(&self, codata_types: &[CodataDeclaration]) -> bool {
        match self {
            Ty::Int => false,
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
            Ty::Int => alloc.keyword(INT),
            Ty::Decl(name) => alloc.typ(name),
        }
    }
}

#[cfg(test)]
mod ty_tests {
    use printer::Print;

    use super::Ty;

    #[test]
    fn print_int() {
        let result = Ty::Int.print_to_string(Default::default());
        assert_eq!(result, "Int")
    }

    #[test]
    fn print_list() {
        let result = Ty::Decl("ListInt".to_string()).print_to_string(Default::default());
        assert_eq!(result, "ListInt")
    }
}
