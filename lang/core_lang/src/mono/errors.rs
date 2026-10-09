//! Defines the errors that can occur during constraint collection, solving, and specialization.

use std::fmt;

use crate::mono::growing_cycle::GrowingCycle;

/// This enum defines the errors that can occur across the monomorphization pipeline: constraint
/// collection (`TypeMismatch`, `UndeclaredType`, `UndefinedFunction`, `UndeclaredXtor`), solving
/// (`PolymorphicRecursion`), and specialization (`NameCollision`). Variants are designed to be
/// specific and to provide human-friendly messages via `Display`.
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

    /// A referenced function was not declared in the program.
    UndefinedFunction(String),

    /// A named xtor was not found on a type declaration (e.g. `List::Cons`).
    UndeclaredXtor {
        type_name: String,
        xtor_name: String,
    },

    /// Detected polymorphic recursion, which is not supported by our monomorphization approach.
    PolymorphicRecursion { cycles: Vec<GrowingCycle> },

    /// Two distinct declarations, xtors, or defs mangled to the very same monomorphic name.
    /// The mangling scheme (`mono::naming_table`) is not injective for every possible source
    /// name, e.g. a user-defined `f_Box` can collide with the generated name for `f[Box]`.
    NameCollision {
        mangled: String,
        first: String,
        second: String,
    },

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
            MonoError::UndefinedFunction(name) => write!(f, "Undefined function: '{}'", name),
            MonoError::UndeclaredXtor {
                type_name,
                xtor_name,
            } => write!(
                f,
                "Undeclared xtor: '{}' has no xtor named '{}'",
                type_name, xtor_name
            ),
            MonoError::NameCollision {
                mangled,
                first,
                second,
            } => write!(
                f,
                "Naming collision: '{}' and '{}' both mangle to the monomorphic name '{}'",
                first, second, mangled
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
