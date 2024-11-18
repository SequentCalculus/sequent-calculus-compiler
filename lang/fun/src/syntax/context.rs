use miette::SourceSpan;
use printer::{
    tokens::{CNT, COLON, TICK},
    DocAllocator, Print,
};

use crate::{
    syntax::{
        types::{OptTyped, Ty},
        Covariable, Variable,
    },
    typing::{errors::Error, symbol_table::SymbolTable},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContextBinding {
    TypedVar { var: Variable, ty: Ty },
    TypedCovar { covar: Covariable, ty: Ty },
}

pub type TypingContext = Vec<ContextBinding>;

impl OptTyped for ContextBinding {
    fn get_type(&self) -> Option<Ty> {
        match self {
            ContextBinding::TypedVar { var: _, ty } => Some(ty.clone()),
            ContextBinding::TypedCovar { covar: _, ty } => Some(ty.clone()),
        }
    }
}

impl Print for ContextBinding {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            ContextBinding::TypedVar { var, ty } => alloc
                .text(var)
                .append(alloc.space())
                .append(COLON)
                .append(alloc.space())
                .append(ty.print(cfg, alloc)),
            ContextBinding::TypedCovar { covar, ty } => alloc
                .text(TICK)
                .append(covar)
                .append(alloc.space())
                .append(CNT)
                .append(alloc.space())
                .append(ty.print(cfg, alloc)),
        }
    }
}

pub fn check_typing_context(ctx: &TypingContext, symbol_table: &SymbolTable) -> Result<(), Error> {
    for binding in ctx {
        match binding {
            ContextBinding::TypedVar { ty, .. } | ContextBinding::TypedCovar { ty, .. } => {
                ty.check(symbol_table)?;
            }
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
            )
            | (
                ContextBinding::TypedCovar { ty: ty_1, .. },
                ContextBinding::TypedCovar { ty: ty_2, .. },
            ) => {
                if ty_1 != ty_2 {
                    return Err(Error::TypingContextMismatch { span: *span });
                }
            }

            (ContextBinding::TypedVar { .. }, ContextBinding::TypedCovar { .. })
            | (ContextBinding::TypedCovar { .. }, ContextBinding::TypedVar { .. }) => {
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

#[cfg(test)]
mod context_tests {
    use super::{check_typing_context, compare_typing_contexts, lookup_covar, lookup_var};
    use crate::{
        parser::util::ToMiette,
        syntax::{context::ContextBinding, types::Ty},
        typing::symbol_table::{Polarity, SymbolTable},
    };
    use codespan::Span;
    use printer::Print;

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

    #[test]
    fn var_lookup() {
        let result = lookup_var(
            &Span::default().to_miette(),
            &vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_int(),
            }],
            &"x".to_owned(),
        )
        .unwrap();
        let expected = Ty::mk_int();
        assert_eq!(result, expected)
    }

    #[test]
    fn var_lookup_fail() {
        let result = lookup_var(&Span::default().to_miette(), &vec![], &"x".to_owned());
        assert!(result.is_err())
    }

    #[test]
    fn covar_lookup() {
        let result = lookup_covar(
            &Span::default().to_miette(),
            &vec![ContextBinding::TypedCovar {
                covar: "a".to_owned(),
                ty: Ty::mk_int(),
            }],
            &"a".to_owned(),
        )
        .unwrap();
        let expected = Ty::mk_int();
        assert_eq!(result, expected)
    }

    #[test]
    fn covar_lookup_fail() {
        let result = lookup_covar(&Span::default().to_miette(), &vec![], &"a".to_owned());
        assert!(result.is_err())
    }

    fn example_contextitem_var() -> ContextBinding {
        ContextBinding::TypedVar {
            var: "x".to_owned(),
            ty: Ty::mk_int(),
        }
    }

    fn example_contextitem_covar() -> ContextBinding {
        ContextBinding::TypedCovar {
            covar: "a".to_owned(),
            ty: Ty::mk_int(),
        }
    }

    #[test]
    fn display_contextitem_var() {
        let result = example_contextitem_var().print_to_string(Default::default());
        let expected = "x : Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_contextitem_covar() {
        let result = example_contextitem_covar().print_to_string(Default::default());
        let expected = "'a :cnt Int";
        assert_eq!(result, expected)
    }
}
