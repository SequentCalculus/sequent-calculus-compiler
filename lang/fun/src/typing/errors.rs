//! Defines Typing errors
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::syntax::{Covar, Name, Var};

/// Errors during type checking
#[derive(Error, Diagnostic, Debug, Clone)]
pub enum Error {
    /// Name was defined multiple times
    #[error("{name} was defined multiple times.")]
    #[diagnostic(code("T-001"))]
    DefinedMultipleTimes {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The name that was defined multiple times
        name: Name,
    },
    /// Name was not defined
    #[error("{name} is undefined.")]
    #[diagnostic(code("T-002"))]
    Undefined {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The undefined name
        name: Name,
    },
    /// Unexpected term or type
    #[error("Expected: {expected}\nGot: {got}")]
    #[diagnostic(code("T-003"))]
    Mismatch {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// What was expected
        expected: String,
        /// What was found
        got: String,
    },
    /// Variable was not bound before use
    #[error("Unbound variable: {var}")]
    #[diagnostic(code("T-004"))]
    UnboundVariable {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The unbound variable
        var: Var,
    },
    /// Covariable was not bound before use
    #[error("Unbound covariable: {covar}")]
    #[diagnostic(code("T-005"))]
    UnboundCovariable {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The unbound covariable
        covar: Covar,
    },
    /// Argument arity mismatch
    #[error("Wrong number of arguments.\nExpected: {expected}\nGot: {got}")]
    #[diagnostic(code("T-006"))]
    WrongNumberOfArguments {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The expected number of arguments
        expected: usize,
        /// The actual number of arguments
        got: usize,
    },
    /// Found a covariable where a term was expected
    #[error("Expected a term argument but found a covariable.")]
    #[diagnostic(code("T-007"))]
    ExpectedTermGotCovariable {
        /// The Source Location
        #[label]
        span: SourceSpan,
    },
    /// Found a term where a covariable was expected
    #[error("Expected a covariable argument but found a term.")]
    #[diagnostic(code("T-008"))]
    ExpectedCovariableGotTerm {
        /// The Source Location
        #[label]
        span: SourceSpan,
    },
    /// Found a case with no patterns
    #[error("Empty matches are not supported.")]
    #[diagnostic(code("T-009"))]
    EmptyMatch {
        /// The Source Location
        #[label]
        span: SourceSpan,
    },
    /// Destructor pattern in new epxression
    #[error("Missing destructors in new expression: {dtor}")]
    #[diagnostic(code("T-010"))]
    MissingDtorInNew {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The destructor name
        dtor: String,
    },
    /// Expected i64, but got a different type
    #[error("Expected type i64 for new expression.")]
    #[diagnostic(code("T-011"))]
    ExpectedI64ForNew {
        /// The Source Location
        #[label]
        span: SourceSpan,
    },
    /// Expected data type but found a different type
    #[error("Expected data type {data} for new expression.")]
    #[diagnostic(code("T-012"))]
    ExpectedDataForNew {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The data type that was expected
        data: Name,
    },
    /// Arity Mismatch in pattern (case or new)
    #[error("Wrong number of binders in clause.\nExpected: {expected}\nProvided: {provided}")]
    #[diagnostic(code("T-013"))]
    WrongNumberOfBinders {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The expected number of arguments
        expected: usize,
        /// The found number of arguments
        provided: usize,
    },
    /// Unexpected chirality or type in context
    #[error("Mismatch in typing context.\nExpected: {expected}\nProvided: {provided}")]
    #[diagnostic(code("T-014"))]
    TypingContextMismatch {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// What was expected
        expected: String,
        /// What as provided
        provided: String,
    },
    /// Missing pattern in case
    #[error("Missing constructor pattern in case expression: {ctor}")]
    #[diagnostic(code("T-015"))]
    MissingCtorInCase {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The missing constructor name
        ctor: String,
    },
    /// Extra patterns in case
    #[error("Unexpected constructors in case expression: {ctors}")]
    #[diagnostic(code("T-016"))]
    UnexpectedCtorsInCase {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The extra constructor name(s)
        ctors: String,
    },
    /// Extra patterns in new
    #[error("Unexpected destructors in new expression: {dtors}")]
    #[diagnostic(code("T-017"))]
    UnexpectedDtorsInNew {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The extra destructor name(s)
        dtors: String,
    },
    /// The same variable was bound more than once
    #[error("{var} is bound multiple times in parameter list of {name}.")]
    #[diagnostic(code("T-018"))]
    VarBoundMultipleTimes {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The variable that was bound multiple times
        var: Var,
        /// The definition in which the variable was used
        name: Name,
    },
    /// The same covariable was bound more than once
    #[error("{covar} is bound multiple times in parameter list {name}.")]
    #[diagnostic(code("T-019"))]
    CovarBoundMultipleTimes {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The covariable that was bound multiple times
        covar: Covar,
        /// The definition in which the covariable was used
        name: Name,
    },
    /// The same type parameter was gbound more than once
    #[error("{param} is bound multiple times in type parameter list of {name}.")]
    #[diagnostic(code("T-020"))]
    TypeParameterBoundMultipleTimes {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The type parameter that was bound multiple times
        param: Name,
        /// The definition in which the type parameter was used
        name: Name,
    },
    /// Expected i64 in constructor but got a different one
    #[error("Expected type i64 for constructor.")]
    #[diagnostic(code("T-021"))]
    ExpectedI64ForConstructor {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The constructor name
        name: Name,
    },
    /// Type Argument Arity Mismatch
    #[error("Wrong number of type arguments.\nExpected: {expected}\nGot: {got}")]
    #[diagnostic(code("T-022"))]
    WrongNumberOfTypeArguments {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The expected number of arguments
        expected: usize,
        /// The actual number of arguments
        got: usize,
    },
    /// Use of undefined name or type arguments
    #[error("{name} is undefined.\nPerhaps the annotated type arguments are wrong: {type_args}")]
    #[diagnostic(code("T-023"))]
    UndefinedWrongTypeArguments {
        /// The Source Location
        #[label]
        span: SourceSpan,
        /// The undefined name
        name: Name,
        /// The provided type arguments
        type_args: String,
    },
}
