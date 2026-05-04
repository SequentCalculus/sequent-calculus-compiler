//! This module contains the errors of the loader.

use thiserror::Error;

use miette::Diagnostic;

#[derive(Error, Diagnostic, Debug, Clone)]
pub enum LoaderError {
    #[error("Unable to find binary {bin_name}")]
    #[diagnostic(code("L-001"))]
    BinaryNotFound { bin_name: String },
    #[error("Unable to find file {path_to_file}")]
    #[diagnostic(code("L-002"))]
    FileNotFound { path_to_file: String },
}
