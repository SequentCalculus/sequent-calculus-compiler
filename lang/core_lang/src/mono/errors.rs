use std::fmt;

use crate::mono::growing_cycle::GrowingCycle;

/// This enum defines the errors that can occur during typechecking and
/// constraint collection. Variants are designed to be specific and to
/// provide human-friendly messages via `Display`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonoError {
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

    /// A referenced function was not declared in the program.
    UndefinedFunction(String),

    /// The number of provided type arguments (or xtor args) does not match
    /// the declared arity.
    ArityMismatch { expected: usize, got: usize },

    /// A named xtor was not found on a type declaration (e.g. `List::Cons`).
    UndeclaredXtor {
        type_name: String,
        xtor_name: String,
    },

    /// Detected polymorphic recursion, which is not supported by our monomorphization approach.
    PolymorphicRecursion { cycles: Vec<GrowingCycle> },

    /// Generic wrapper for other errors with contextual message.
    Contextual { msg: String },
}

impl fmt::Display for MonoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MonoError::TypeMismatch { expected, got, msg } => {
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
            MonoError::UndeclaredType(name) => write!(f, "Undeclared type: '{}'", name),
            MonoError::UndeclaredVariable(name) => write!(f, "Undeclared variable: '{}'", name),
            MonoError::UndefinedFunction(name) => write!(f, "Undefined function: '{}'", name),
            MonoError::ArityMismatch { expected, got } => write!(
                f,
                "Arity mismatch: expected {} arguments but got {}",
                expected, got
            ),
            MonoError::UndeclaredXtor {
                type_name,
                xtor_name,
            } => write!(
                f,
                "Undeclared xtor: '{}' has no xtor named '{}'",
                type_name, xtor_name
            ),
            MonoError::Contextual { msg } => write!(f, "{}", msg),
            MonoError::PolymorphicRecursion { cycles } => {
                write!(
                    f,
                    "Polymorphic recursion detected in cycles: {}",
                    cycles
                        .iter()
                        .map(|cycle| format!("\n{}", cycle))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
        }
    }
}

impl std::error::Error for MonoError {}
