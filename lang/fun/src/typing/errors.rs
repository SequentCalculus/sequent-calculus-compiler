use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::syntax::{types::Ty, Covariable, Name, Variable};

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
    #[error("Unbound covariable: '{covar}")]
    #[diagnostic(code("T-005"))]
    UnboundCovariable {
        #[label]
        span: SourceSpan,
        covar: Covariable,
    },
    #[error("Wrong number of arguments.\nExpected: {expected}\nGot: {got}")]
    #[diagnostic(code("T-006"))]
    WrongNumberOfArguments {
        #[label]
        span: SourceSpan,
        expected: usize,
        got: usize,
    },
    #[error("Expected a term argument but found a covariable.")]
    #[diagnostic(code("T-007"))]
    ExpectedTermGotCovariable {
        #[label]
        span: SourceSpan,
    },
    #[error("Expected a covariable argument but found a term.")]
    #[diagnostic(code("T-008"))]
    ExpectedCovariableGotTerm {
        #[label]
        span: SourceSpan,
    },
    #[error("Empty matches are not supported.")]
    #[diagnostic(code("T-009"))]
    EmptyMatch {
        #[label]
        span: SourceSpan,
    },
}
