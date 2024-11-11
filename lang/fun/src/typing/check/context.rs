use crate::{
    syntax::context::{ContextBinding, TypingContext},
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
