use crate::syntax::{types::Ty, Name, Variable};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    DefinedMultipleTimes(Name),
    Undefined(Name),
    Mismatch { expected: Ty, got: Ty },
    UnboundVariable { var: Variable },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DefinedMultipleTimes(name) => write!(f, "{name} was defined mutliple times."),
            Error::Undefined(name) => write!(f, "{name} is undefined."),
            Error::Mismatch { expected, got } => write!(f, "Expected: {expected} Got: {got}"),
            Error::UnboundVariable { var } => write!(f, "Unbound variable: {var}"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod error_tests {
    use super::Error;

    #[test]
    fn display_defined_multiple() {
        let result = format!("{}", Error::DefinedMultipleTimes("List".to_owned()));
        let expected = "List was defined mutliple times.";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_undefined() {
        let result = format!("{}", Error::Undefined("List".to_owned()));
        let expected = "List is undefined.";
        assert_eq!(result, expected)
    }
}
