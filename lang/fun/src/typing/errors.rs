use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::syntax::{Covar, Name, Var};

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
        expected: String,
        got: String,
    },
    #[error("Unbound variable: {var}")]
    #[diagnostic(code("T-004"))]
    UnboundVariable {
        #[label]
        span: SourceSpan,
        var: Var,
    },
    #[error("Unbound covariable: {covar}")]
    #[diagnostic(code("T-005"))]
    UnboundCovariable {
        #[label]
        span: SourceSpan,
        covar: Covar,
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
    #[error("Missing destructors in cocase expression: {dtor}")]
    #[diagnostic(code("T-010"))]
    MissingDtorInCocase {
        #[label]
        span: SourceSpan,
        dtor: String,
    },
    #[error("Expected type i64 for cocase expression.")]
    #[diagnostic(code("T-011"))]
    ExpectedI64ForCocase {
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
    #[error("Mismatch in typing context.\nExpected: {expected}\nProvided: {provided}")]
    #[diagnostic(code("T-014"))]
    TypingContextMismatch {
        #[label]
        span: SourceSpan,
        expected: String,
        provided: String,
    },
    #[error("Missing constructor pattern in case expression: {ctor}")]
    #[diagnostic(code("T-015"))]
    MissingCtorInCase {
        #[label]
        span: SourceSpan,
        ctor: String,
    },
    #[error("Unexpected constructors in case expression: {ctors}")]
    #[diagnostic(code("T-016"))]
    UnexpectedCtorsInCase {
        #[label]
        span: SourceSpan,
        ctors: String,
    },
    #[error("Unexpected destructors in cocase expression: {dtors}")]
    #[diagnostic(code("T-017"))]
    UnexpectedDtorsInCocase {
        #[label]
        span: SourceSpan,
        dtors: String,
    },
    #[error("{var} is bound multiple times in parameter list of {name}.")]
    #[diagnostic(code("T-018"))]
    VarBoundMultipleTimes {
        #[label]
        span: SourceSpan,
        var: Var,
        name: Name,
    },
    #[error("{covar} is bound multiple times in parameter list {name}.")]
    #[diagnostic(code("T-019"))]
    CovarBoundMultipleTimes {
        #[label]
        span: SourceSpan,
        covar: Covar,
        name: Name,
    },
    #[error("{param} is bound multiple times in type parameter list of {name}.")]
    #[diagnostic(code("T-020"))]
    TypeParameterBoundMultipleTimes {
        #[label]
        span: SourceSpan,
        param: Name,
        name: Name,
    },
    #[error("Expected type i64 for constructor.")]
    #[diagnostic(code("T-021"))]
    ExpectedI64ForConstructor {
        #[label]
        span: SourceSpan,
        name: Name,
    },
    #[error("Wrong number of type arguments.\nExpected: {expected}\nGot: {got}")]
    #[diagnostic(code("T-022"))]
    WrongNumberOfTypeArguments {
        #[label]
        span: SourceSpan,
        expected: usize,
        got: usize,
    },
    #[error("{name} is undefined.\nPerhaps the annotated type arguments are wrong: {type_args}")]
    #[diagnostic(code("T-023"))]
    UndefinedWrongTypeArguments {
        #[label]
        span: SourceSpan,
        name: Name,
        type_args: String,
    },
}
