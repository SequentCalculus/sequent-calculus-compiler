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
    #[error("Missing destructors in cocase expression.")]
    #[diagnostic(code("T-010"))]
    MissingDtorInCocase {
        #[label]
        span: SourceSpan,
    },
    #[error("Expected type Int for cocase expression.")]
    #[diagnostic(code("T-011"))]
    ExpectedIntForCocase {
        #[label]
        span: SourceSpan,
    },
    #[error("Expected data type {data} for cocase expression.")]
    #[diagnostic(code("T-012"))]
    ExpectedDataForCocase {
        #[label]
        span: SourceSpan,
        data: Name,
    },
    #[error("Wrong number of binders in clause.\nExpected: {expected}\nProvided: {provided}")]
    #[diagnostic(code("T-013"))]
    WrongNumberOfBinders {
        #[label]
        span: SourceSpan,
        expected: usize,
        provided: usize,
    },
    #[error("Mismatch in expected and provided typing context.")]
    #[diagnostic(code("T-014"))]
    TypingContextMismatch {
        #[label]
        span: SourceSpan,
    },
    #[error("Missing constructor patterns in case expression.")]
    #[diagnostic(code("T-015"))]
    MissingCtorsInCase {
        #[label]
        span: SourceSpan,
    },
    #[error("Unexpected constructor {ctor} in case expression")]
    #[diagnostic(code("T-016"))]
    UnexpectedCtorInCase {
        #[label]
        span: SourceSpan,
        ctor: String,
    },
    #[error("Unexpected destructor {dtor} in cocase expression")]
    #[diagnostic(code("T-017"))]
    UnexpectedDtorInCocase {
        #[label]
        span: SourceSpan,
        dtor: String,
    },
}
