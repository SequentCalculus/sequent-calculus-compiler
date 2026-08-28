use std::fmt;
use std::panic::Location;

use crate::syntax::type_params::ParamPolarity;

/// This macro captures the source location (file and line) precisely where it is called,
/// constructs a `LocatedError`, and returns it immediately as an `Err` variant.
///
/// # Example
/// ```
/// use scc_core_lang::bail;
/// use scc_core_lang::typing::errors::TypeError;
/// fn try_bail() -> Result<(), scc_core_lang::typing::errors::LocatedTypeError> {
///     bail!(TypeError::UndeclaredVariable("foo".to_string()));
/// }
/// ```
#[macro_export]
macro_rules! bail {
    ($err:expr) => {
        return Err($crate::typing::errors::LocatedTypeError::new($err))
    };
}

/// A wrapper that pairs a specific `TypeError` with its exact instantiation point
/// within the Rust compiler source code. This is what your typechecking functions
/// should return as their `Err` variant (e.g., `Result<Ty, LocatedError>`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocatedTypeError {
    pub error: TypeError,
    pub location: &'static Location<'static>,
}

impl LocatedTypeError {
    /// Creates a new `LocatedError` and automatically captures the file and line number
    /// of the caller using `#[track_caller]`.
    #[track_caller]
    pub fn new(error: TypeError) -> Self {
        Self {
            error,
            location: Location::caller(),
        }
    }
}

/// This enum defines the errors that can occur during typechecking and
/// constraint collection. Variants are designed to be specific and to
/// provide human-friendly messages via `Display`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeError {
    /// A concrete type does not match the expected type.
    TypeMismatch {
        expected: String,
        got: String,
        msg: Option<String>,
    },

    /// A referenced type name was not declared in the program.
    UndeclaredType(String),

    /// A referenced variable/covariable was not declared in the current scope.
    UndeclaredVariable(String),

    /// The number of provided type arguments (or xtor args) does not match
    /// the declared arity.
    ArityMismatch { expected: usize, got: usize },

    /// A named xtor was not found on a type declaration (e.g. `List::Cons`).
    UndeclaredXtor {
        type_name: String,
        xtor_name: String,
    },

    /// A referenced function was not defined.
    UndefinedFunction(String),

    /// A type name was declared more than once in the program.
    DuplicateTypeName(String),

    /// A function name was declared more than once in the program.
    DuplicateDefName(String),

    /// An xtor name was declared more than once across all data/codata declarations.
    DuplicateXtorName(String),

    /// A type argument's polarity does not match the declared polarity of the corresponding
    /// type parameter (`data` = positive/CBV, `codata` = negative/CBN).
    PolarityMismatch {
        expected: ParamPolarity,
        got: ParamPolarity,
    },

    /// Generic wrapper for other errors with contextual message.
    Contextual { msg: String },
}

// Formats the error message for the end-user, seamlessly appending
// the internal Rust source location at the end for compiler debugging.
impl fmt::Display for LocatedTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (at {}:{})",
            self.error,
            self.location.file(),
            self.location.line()
        )
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::TypeMismatch { expected, got, msg } => {
                if let Some(m) = msg {
                    write!(
                        f,
                        "Type mismatch: expected '{}' but got '{}'. \n{}",
                        expected, got, m
                    )
                } else {
                    write!(
                        f,
                        "Type mismatch: expected '{}' but got '{}'",
                        expected, got
                    )
                }
            }
            TypeError::UndeclaredType(name) => write!(f, "Undeclared type: '{}'", name),
            TypeError::UndeclaredVariable(name) => write!(f, "Undeclared variable: '{}'", name),
            TypeError::ArityMismatch { expected, got } => write!(
                f,
                "Arity mismatch: expected {} arguments but got {}",
                expected, got
            ),
            TypeError::UndeclaredXtor {
                type_name,
                xtor_name,
            } => write!(
                f,
                "Undeclared xtor: '{}' has no xtor named '{}'",
                type_name, xtor_name
            ),
            TypeError::UndefinedFunction(name) => write!(f, "Undefined function: '{}'", name),
            TypeError::DuplicateTypeName(name) => write!(f, "Duplicate type name: '{}'", name),
            TypeError::DuplicateDefName(name) => write!(f, "Duplicate function name: '{}'", name),
            TypeError::DuplicateXtorName(name) => {
                write!(f, "Duplicate xtor name: '{}'", name,)
            }
            TypeError::PolarityMismatch { expected, got } => write!(
                f,
                "Polarity mismatch: expected {} but got {}",
                expected, got
            ),
            TypeError::Contextual { msg } => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for LocatedTypeError {}
