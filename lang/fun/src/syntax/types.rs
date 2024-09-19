use crate::syntax::Name;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Int(),
    Decl(Name),
}
impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Int() => write!(f, "Int"),
            Ty::Decl(name) => write!(f, "{}", name),
        }
    }
}

#[cfg(test)]
mod type_tests {
    use super::Ty;

    #[test]
    fn display_int() {
        assert_eq!(format!("{}", Ty::Int()), "Int".to_owned())
    }
}
