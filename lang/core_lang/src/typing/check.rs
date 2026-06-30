use crate::{
    bail,
    syntax::{Identifier, TypingContext},
    typing::{
        env::GlobalEnv,
        errors::{LocatedTypeError, TypeError},
    },
};

/// This trait defines the type checking behavior for all syntax elements in core. The `check` method takes the current context of type parameters, data declarations, codata declarations, and function definitions, and returns an error if the syntax element is not well-typed.
pub trait Checked: Sized {
    fn check(
        &self,
        type_params: &[Identifier],
        context: &TypingContext,
        env: &GlobalEnv,
    ) -> Result<(), LocatedTypeError>;
}

pub fn check_arity(expected: usize, got: usize) -> Result<(), LocatedTypeError> {
    if expected != got {
        bail!(TypeError::ArityMismatch { expected, got })
    }
    Ok(())
}
