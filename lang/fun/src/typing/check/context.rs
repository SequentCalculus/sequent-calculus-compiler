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

#[cfg(test)]
mod context_tests {
    use super::{check_typing_context, compare_typing_contexts};
    use crate::{
        parser::util::ToMiette,
        syntax::{context::ContextBinding, types::Ty},
        typing::symbol_table::{Polarity, SymbolTable},
    };
    use codespan::Span;

    #[test]
    fn context_check() {
        let mut symbol_table = SymbolTable::default();
        symbol_table
            .ty_ctors
            .insert("FunIntInt".to_owned(), (Polarity::Codata, vec![]));
        let result = check_typing_context(
            &vec![
                ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_int(),
                },
                ContextBinding::TypedCovar {
                    covar: "a".to_owned(),
                    ty: Ty::mk_decl("FunIntInt"),
                },
            ],
            &symbol_table,
        );
        assert!(result.is_ok())
    }

    #[test]
    fn context_check_fail() {
        let result = check_typing_context(
            &vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_decl("ListInt"),
            }],
            &SymbolTable::default(),
        );
        assert!(result.is_err())
    }

    #[test]
    fn context_compare() {
        let result = compare_typing_contexts(
            &Span::default().to_miette(),
            &vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_int(),
            }],
            &vec![ContextBinding::TypedVar {
                var: "y".to_owned(),
                ty: Ty::mk_int(),
            }],
        );
        assert!(result.is_ok())
    }

    #[test]
    fn context_compare_fail() {
        let result = compare_typing_contexts(
            &Span::default().to_miette(),
            &vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_int(),
            }],
            &vec![ContextBinding::TypedCovar {
                covar: "a".to_owned(),
                ty: Ty::mk_int(),
            }],
        );
        assert!(result.is_err())
    }
}
