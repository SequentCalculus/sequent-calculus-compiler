use crate::errors::Error;
use std::{fmt, str::FromStr};

#[derive(Clone, Copy, Debug)]
pub enum Method {
    GotoDefinition,
    DidOpen,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Method::GotoDefinition => f.write_str("textDocument/definition"),
            Method::DidOpen => f.write_str("textDocument/didOpen"),
        }
    }
}

impl FromStr for Method {
    type Err = Error;
    fn from_str(s: &str) -> Result<Method, Self::Err> {
        let goto_str = Method::GotoDefinition.to_string();
        let didopen_str = Method::DidOpen.to_string();
        if s == &goto_str {
            Ok(Method::GotoDefinition)
        } else if s == &didopen_str {
            Ok(Method::DidOpen)
        } else {
            Err(Error::UnsupportedMethod(s.to_owned()))
        }
    }
}
