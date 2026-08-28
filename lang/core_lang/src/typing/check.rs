use crate::{
    bail,
    syntax::{TypingContext, type_params::ParamPolarity, type_params::TypeParam},
    typing::{
        env::GlobalEnv,
        errors::{LocatedTypeError, TypeError},
    },
};

/// This trait defines the type checking behavior for all syntax elements in core. The `check` method takes the current context of type parameters (with their declared polarity), data declarations, codata declarations, and function definitions, and returns an error if the syntax element is not well-typed.
pub trait Checked: Sized {
    fn check(
        &self,
        type_params: &[TypeParam],
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

/// Checks that a type argument's actual polarity matches the declared polarity of the
/// corresponding type parameter, mirroring `check_arity` but for polarity (`data`/`+` = positive,
/// `codata`/`-` = negative). Called once per type argument at every instantiation site, alongside
/// `check_arity`.
pub fn check_polarity(expected: ParamPolarity, got: ParamPolarity) -> Result<(), LocatedTypeError> {
    if expected != got {
        bail!(TypeError::PolarityMismatch { expected, got })
    }
    Ok(())
}
