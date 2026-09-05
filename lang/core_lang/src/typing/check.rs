use crate::{
    bail,
    syntax::{Ty, TypingContext, type_params::ParamPolarity, type_params::TypeParam},
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

/// Checks a sequence of type arguments against the declaration-site type parameters they
/// instantiate: arity, then per argument well-formedness (via [`Checked::check`]) and that the
/// argument's own polarity matches the parameter's declared one.
///
/// `type_params` is the ambient list of declaration-site type parameters currently in scope (see
/// [`Checked::check`]), needed to resolve a `Ty::Var` argument's own polarity; it is unrelated to
/// `declared_params`, the parameters being instantiated here.
pub fn check_type_args(
    args: &[Ty],
    declared_params: &[TypeParam],
    type_params: &[TypeParam],
    context: &TypingContext,
    env: &GlobalEnv,
) -> Result<(), LocatedTypeError> {
    check_arity(declared_params.len(), args.len())?;
    for (arg, declared_param) in args.iter().zip(declared_params) {
        arg.check(type_params, context, env)?;
        let got = if arg.is_codata(env.codata_decls, type_params) {
            ParamPolarity::Codata
        } else {
            ParamPolarity::Data
        };
        check_polarity(declared_param.polarity, got)?;
    }
    Ok(())
}
