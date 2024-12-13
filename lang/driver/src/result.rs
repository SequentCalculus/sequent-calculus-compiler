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
    #[error("Unable to find binary {bin_name}")]
    #[diagnostic(code("D-001"))]
    BinaryNotFound { bin_name: String },
}
