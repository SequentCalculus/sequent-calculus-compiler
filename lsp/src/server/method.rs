use crate::errors::Error;
use std::{fmt, str::FromStr};

#[derive(Clone, Copy)]
pub enum Method {
    GotoDefinition,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Method::GotoDefinition => f.write_str("textDocument/definition"),
        }
    }
}

impl FromStr for Method {
    type Err = Error;
    fn from_str(s: &str) -> Result<Method, Self::Err> {
        let goto_str = Method::GotoDefinition.to_string();
        if s == &goto_str {
            Ok(Method::GotoDefinition)
        } else {
            Err(Error::UnsupportedMethod(s.to_owned()))
        }
    }
}
