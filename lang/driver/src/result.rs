use std::fmt::Display;

use thiserror::Error;

use miette::Diagnostic;

#[derive(Error, Diagnostic, Debug, Clone)]
pub enum DriverError {
    ParseError(fun::parser::result::ParseError),
    TypeError(fun::typing::errors::Error),
}

impl Display for DriverError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
