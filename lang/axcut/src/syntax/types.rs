use printer::{theme::ThemeExt, tokens::INT, Print};

use super::names::Name;

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

#[cfg(test)]
mod types_tests {
    use super::Ty;
    use printer::Print;

    #[test]
    fn print_int() {
        let result = Ty::Int.print_to_string(Default::default());
        let expected = "Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn print_decl() {
        let result = Ty::Decl("ListInt".to_owned()).print_to_string(Default::default());
        let expected = "ListInt";
        assert_eq!(result, expected)
    }
}
