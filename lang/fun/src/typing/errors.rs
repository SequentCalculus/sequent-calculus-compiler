use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::syntax::{types::Ty, Name, Variable};

#[derive(Error, Diagnostic, Debug, Clone)]
pub enum Error {
    #[error("{name} was defined multiple times.")]
    #[diagnostic(code("T-001"))]
    DefinedMultipleTimes {
        #[label]
        span: SourceSpan,
        name: Name,
    },
    #[error("{name} is undefined.")]
    #[diagnostic(code("T-002"))]
    Undefined {
        #[label]
        span: SourceSpan,
        name: Name,
    },
    #[error("Expected: {expected}\nGot: {got}")]
    #[diagnostic(code("T-003"))]
    Mismatch {
        #[label]
        span: SourceSpan,
        expected: Ty,
        got: Ty,
    },
    #[error("Unbound variable: {var}")]
    #[diagnostic(code("T-004"))]
    UnboundVariable {
        #[label]
        span: SourceSpan,
        var: Variable,
    },
}
