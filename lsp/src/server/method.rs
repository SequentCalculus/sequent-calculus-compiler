use crate::errors::Error;
use std::{fmt, str::FromStr};

#[derive(Clone, Copy, Debug)]
pub enum Method {
    GotoDefinition,
    GotoImplementation,
    Formatter,
    DidOpen,
    DidChange,
    PublishDiagnostics,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Method::GotoDefinition => f.write_str("textDocument/definition"),
            Method::GotoImplementation => f.write_str("textDocument/implementation"),
            Method::Formatter => f.write_str("textDocument/formatting"),
            Method::DidOpen => f.write_str("textDocument/didOpen"),
            Method::DidChange => f.write_str("textDocument/didChange"),
            Method::PublishDiagnostics => f.write_str("textDocument/publishDiagnostics"),
        }
    }
}

impl FromStr for Method {
    type Err = Error;
    fn from_str(s: &str) -> Result<Method, Self::Err> {
        let s = s.trim();
        let methods = [
            Method::GotoDefinition,
            Method::GotoImplementation,
            Method::DidOpen,
            Method::DidChange,
            Method::PublishDiagnostics,
        ];
        for method in methods {
            if method.to_string() == s {
                return Ok(method);
            }
        }
        Err(Error::UnsupportedMethod(s.to_owned()))
    }
}
