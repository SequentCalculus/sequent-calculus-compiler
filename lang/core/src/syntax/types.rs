use printer::{tokens::INT, DocAllocator, Print};

use super::Name;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ty {
    Int(),
    Decl(Name),
}

impl Print for Ty {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Ty::Int() => alloc.text(INT),
            Ty::Decl(name) => alloc.text(name),
        }
    }
}

#[cfg(test)]
mod ty_tests {
    use printer::Print;

    use super::Ty;

    #[test]
    fn print_int() {
        let result = Ty::Int().print_to_string(Default::default());
        assert_eq!(result, "Int")
    }

    #[test]
    fn print_list() {
        let result = Ty::Decl("ListInt".to_owned()).print_to_string(Default::default());
        assert_eq!(result, "ListInt")
    }
}
