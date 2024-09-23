use super::Name;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ty {
    Int(),
    Decl(Name),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ty::Int() => f.write_str("Int"),
            Ty::Decl(name) => f.write_str(name),
        }
    }
}

#[cfg(test)]
mod ty_tests {
    use super::Ty;

    #[test]
    fn display_int() {
        let result = format!("{}", Ty::Int());
        let expected = "Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_list() {
        let result = format!("{}", Ty::Decl("ListInt".to_owned()));
        let expected = "ListInt";
        assert_eq!(result, expected)
    }
}
