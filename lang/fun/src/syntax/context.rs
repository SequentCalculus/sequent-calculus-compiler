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

// Context Bindings
//
//

/// Describes a single binding that can occur in a typing context.
/// Either
/// - A variable binding: `x: ty`
/// - A covariable binding `'a :cns ty`
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContextBinding {
    TypedVar { var: Variable, ty: Ty },
    TypedCovar { covar: Covariable, ty: Ty },
}

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

// TypingContext
//
//

/// A typing context.
/// Example:
/// `x: Int, y: ListInt`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypingContext {
    pub bindings: Vec<ContextBinding>,
}

impl Print for TypingContext {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.bindings.is_empty() {
            alloc.nil()
        } else {
            self.bindings.print(cfg, alloc).parens()
        }
    }
}

impl TypingContext {
    /// Check whether all types in the typing context are valid.
    pub fn check(&self, symbol_table: &SymbolTable) -> Result<(), Error> {
        for binding in self.bindings.iter() {
            match binding {
                ContextBinding::TypedVar { ty, .. } | ContextBinding::TypedCovar { ty, .. } => {
                    ty.check(symbol_table)?;
                }
            }
        }
        Ok(())
    }

    /// Lookup the type of a variable in the context.
    pub fn lookup_var(&self, span: &SourceSpan, searched_var: &Variable) -> Result<Ty, Error> {
        // Due to variable shadowing we have to traverse from
        // right to left.
        for binding in self.bindings.iter().rev() {
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

    /// Lookup the type of a covariable in the context.
    pub fn lookup_covar(
        &self,
        span: &SourceSpan,
        searched_covar: &Covariable,
    ) -> Result<Ty, Error> {
        // Due to variable shadowing we have to traverse from
        // right to left.
        for binding in self.bindings.iter().rev() {
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

    /// Check whether the typing context corresponds to the expected one.
    pub fn compare_to(&self, span: &SourceSpan, expected: &TypingContext) -> Result<(), Error> {
        if self.bindings.len() != expected.bindings.len() {
            return Err(Error::WrongNumberOfBinders {
                span: *span,
                expected: expected.bindings.len(),
                provided: self.bindings.len(),
            });
        }
        for x in self.bindings.iter().zip(expected.bindings.iter()) {
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
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::util::ToMiette,
        syntax::{
            context::{ContextBinding, TypingContext},
            types::Ty,
        },
        typing::symbol_table::{Polarity, SymbolTable},
    };
    use codespan::Span;
    use printer::Print;

    /// The context:
    /// `x: Int, y: ListInt, 'a :cnt Int`
    fn example_context() -> TypingContext {
        TypingContext {
            bindings: vec![
                ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_int(),
                },
                ContextBinding::TypedVar {
                    var: "y".to_owned(),
                    ty: Ty::mk_decl("ListInt"),
                },
                ContextBinding::TypedCovar {
                    covar: "a".to_owned(),
                    ty: Ty::mk_int(),
                },
            ],
        }
    }

    // Checking prettyprinting
    //
    //

    #[test]
    fn print_context() {
        assert_eq!(
            example_context().print_to_string(None),
            "x: Int, y: ListInt, 'a :cnt Int"
        )
    }

    #[test]
    fn print_context_empty() {
        assert_eq!(TypingContext { bindings: vec![] }.print_to_string(None), "")
    }

    // Checking well-formedness of contexts
    //
    //

    #[test]
    fn context_check() {
        let mut symbol_table = SymbolTable::default();
        symbol_table
            .ty_ctors
            .insert("ListInt".to_owned(), (Polarity::Data, vec![]));
        assert!(example_context().check(&symbol_table).is_ok())
    }
    #[test]
    fn context_check_fail() {
        assert!(example_context().check(&SymbolTable::default()).is_err())
    }

    // Comparing two contexts
    //
    //

    // #[test]
    // fn context_compare() {
    //     let result = compare_typing_contexts(
    //         &Span::default().to_miette(),
    //         &vec![ContextBinding::TypedVar {
    //             var: "x".to_owned(),
    //             ty: Ty::mk_int(),
    //         }],
    //         &vec![ContextBinding::TypedVar {
    //             var: "y".to_owned(),
    //             ty: Ty::mk_int(),
    //         }],
    //     );
    //     assert!(result.is_ok())
    // }
    // #[test]
    // fn context_compare_fail() {
    //     let result = compare_typing_contexts(
    //         &Span::default().to_miette(),
    //         &vec![ContextBinding::TypedVar {
    //             var: "x".to_owned(),
    //             ty: Ty::mk_int(),
    //         }],
    //         &vec![ContextBinding::TypedCovar {
    //             covar: "a".to_owned(),
    //             ty: Ty::mk_int(),
    //         }],
    //     );
    //     assert!(result.is_err())
    // }

    // Checking variable and covariable lookup
    //
    //

    #[test]
    fn var_lookup() {
        assert!(example_context()
            .lookup_var(&Span::default().to_miette(), &"x".to_owned())
            .is_ok())
    }

    #[test]
    fn var_lookup_fail() {
        assert!(example_context()
            .lookup_var(&Span::default().to_miette(), &"z".to_owned())
            .is_err())
    }

    #[test]
    fn covar_lookup() {
        assert!(example_context()
            .lookup_covar(&Span::default().to_miette(), &"a".to_owned())
            .is_ok())
    }

    #[test]
    fn covar_lookup_fail() {
        assert!(example_context()
            .lookup_covar(&Span::default().to_miette(), &"b".to_owned())
            .is_err())
    }
}
