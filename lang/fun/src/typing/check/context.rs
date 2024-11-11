use crate::{
    syntax::{
        context::{ContextBinding, TypingContext},
        types::Ty,
        Covariable, Variable,
    },
    typing::{check::check_type, errors::Error, symbol_table::SymbolTable},
};
use miette::SourceSpan;

pub fn check_typing_context(ctx: &TypingContext, symbol_table: &SymbolTable) -> Result<(), Error> {
    for binding in ctx.iter() {
        match binding {
            ContextBinding::TypedVar { ty, .. } => check_type(ty, symbol_table)?,
            ContextBinding::TypedCovar { ty, .. } => check_type(ty, symbol_table)?,
        }
    }
    Ok(())
}

pub fn compare_typing_contexts(
    span: &SourceSpan,
    expected: &TypingContext,
    provided: &TypingContext,
) -> Result<(), Error> {
    if expected.len() != provided.len() {
        return Err(Error::WrongNumberOfBinders {
            span: *span,
            expected: expected.len(),
            provided: provided.len(),
        });
    }
    for x in expected.iter().zip(provided.iter()) {
        match x {
            (
                ContextBinding::TypedVar { ty: ty_1, .. },
                ContextBinding::TypedVar { ty: ty_2, .. },
            ) => {
                if ty_1 != ty_2 {
                    return Err(Error::TypingContextMismatch { span: *span });
                }
            }
            (
                ContextBinding::TypedCovar { ty: ty_1, .. },
                ContextBinding::TypedCovar { ty: ty_2, .. },
            ) => {
                if ty_1 != ty_2 {
                    return Err(Error::TypingContextMismatch { span: *span });
                }
            }

            (ContextBinding::TypedVar { .. }, ContextBinding::TypedCovar { .. }) => {
                return Err(Error::TypingContextMismatch { span: *span })
            }
            (ContextBinding::TypedCovar { .. }, ContextBinding::TypedVar { .. }) => {
                return Err(Error::TypingContextMismatch { span: *span })
            }
        }
    }
    Ok(())
}

pub fn lookup_var(
    span: &SourceSpan,
    ctx: &TypingContext,
    searched_var: &Variable,
) -> Result<Ty, Error> {
    // Due to variable shadowing we have to traverse from
    // right to left.
    for binding in ctx.iter().rev() {
        match binding {
            ContextBinding::TypedVar { var, ty } => {
                if var == searched_var {
                    return Ok(ty.clone());
                }
                continue;
            }
            ContextBinding::TypedCovar { .. } => continue,
        }
    }
    Err(Error::UnboundVariable {
        span: *span,
        var: searched_var.clone(),
    })
}

pub fn lookup_covar(
    span: &SourceSpan,
    ctx: &TypingContext,
    searched_covar: &Covariable,
) -> Result<Ty, Error> {
    // Due to variable shadowing we have to traverse from
    // right to left.
    for binding in ctx.iter().rev() {
        match binding {
            ContextBinding::TypedVar { .. } => continue,
            ContextBinding::TypedCovar { covar, ty } => {
                if covar == searched_covar {
                    return Ok(ty.clone());
                }
                continue;
            }
        }
    }
    Err(Error::UnboundCovariable {
        span: *span,
        covar: searched_covar.clone(),
    })
}
