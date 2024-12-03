use thiserror::Error;

use miette::Diagnostic;

#[derive(Error, Diagnostic, Debug, Clone)]
pub enum DriverError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    ParseError(#[from] fun::parser::result::ParseError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    TypeError(#[from] fun::typing::errors::Error),
}
