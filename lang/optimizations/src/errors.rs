use miette::Diagnostic;
use std::fmt;

#[derive(Debug, Clone, Diagnostic)]
pub enum Error {
    UnknownDeclaration { name: String },
}

impl Error {
    pub fn unknown(name: &str) -> Error {
        Error::UnknownDeclaration {
            name: name.to_owned(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnknownDeclaration { name } => {
                write!(f, "Could not find a type definition for {name}")
            }
        }
    }
}

impl std::error::Error for Error {}
