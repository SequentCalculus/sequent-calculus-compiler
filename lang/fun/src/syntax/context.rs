use codespan::Span;
use derivative::Derivative;
use printer::{
    tokens::{CNT, COLON, TICK},
    DocAllocator, Print,
};

use crate::{
    parser::util::ToMiette,
    syntax::{
        types::{OptTyped, Ty},
        Covariable, Name, Variable,
    },
    typing::{errors::Error, symbol_table::SymbolTable},
};

use std::collections::HashSet;

// Context Bindings
//
//

/// Describes a single binding that can occur in a typing context.
/// Either
/// - A variable binding: `x: ty`
/// - A covariable binding `'a :cns ty`
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct TypingContext {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
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

    /// Check whether no variable in the typing context is duplicated.
    pub fn no_dups(&self, binding_site: Name) -> Result<(), Error> {
        let mut vars: HashSet<Variable> = HashSet::new();
        let mut covars: HashSet<Covariable> = HashSet::new();
        for binding in self.bindings.iter() {
            match binding {
                ContextBinding::TypedVar { var, .. } => {
                    if vars.contains(var) {
                        return Err(Error::VarBoundMultipleTimes {
                            span: self.span.to_miette(),
                            var: var.clone(),
                            name: binding_site,
                        });
                    }
                    vars.insert(var.clone());
                }
                ContextBinding::TypedCovar { covar, .. } => {
                    if covars.contains(covar) {
                        return Err(Error::CovarBoundMultipleTimes {
                            span: self.span.to_miette(),
                            covar: covar.clone(),
                            name: binding_site,
                        });
                    }
                    covars.insert(covar.clone());
                }
            }
        }
        Ok(())
    }

    /// Lookup the type of a variable in the context.
    pub fn lookup_var(&self, searched_var: &Variable) -> Result<Ty, Error> {
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
            span: self.span.to_miette(),
            var: searched_var.clone(),
        })
    }

    /// Lookup the type of a covariable in the context.
    pub fn lookup_covar(&self, searched_covar: &Covariable) -> Result<Ty, Error> {
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
            span: self.span.to_miette(),
            covar: searched_covar.clone(),
        })
    }

    /// Check whether the typing context corresponds to the expected one.
    pub fn compare_to(&self, expected: &TypingContext) -> Result<(), Error> {
        if self.bindings.len() != expected.bindings.len() {
            return Err(Error::WrongNumberOfBinders {
                span: self.span.to_miette(),
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
                        return Err(Error::TypingContextMismatch {
                            span: self.span.to_miette(),
                        });
                    }
                }

                (ContextBinding::TypedVar { .. }, ContextBinding::TypedCovar { .. })
                | (ContextBinding::TypedCovar { .. }, ContextBinding::TypedVar { .. }) => {
                    return Err(Error::TypingContextMismatch {
                        span: self.span.to_miette(),
                    })
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
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
            span: Span::default(),
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

    fn example_context_dup() -> TypingContext {
        TypingContext {
            span: Span::default(),
            bindings: vec![
                ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_int(),
                },
                ContextBinding::TypedCovar {
                    covar: "a".to_owned(),
                    ty: Ty::mk_int(),
                },
                ContextBinding::TypedVar {
                    var: "x".to_owned(),
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
            "(x: Int, y: ListInt, 'a :cnt Int)"
        )
    }

    #[test]
    fn print_context_empty() {
        assert_eq!(
            TypingContext {
                span: Span::default(),
                bindings: vec![]
            }
            .print_to_string(None),
            ""
        )
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
    #[test]
    fn context_check_fail_dup() {
        assert!(example_context_dup()
            .no_dups("binding site".to_string())
            .is_err())
    }

    // Comparing two contexts
    //
    //

    #[test]
    fn context_compare() {
        let result = TypingContext {
            span: Span::default(),
            bindings: vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_int(),
            }],
        }
        .compare_to(&TypingContext {
            span: Span::default(),
            bindings: vec![ContextBinding::TypedVar {
                var: "y".to_owned(),
                ty: Ty::mk_int(),
            }],
        });
        assert!(result.is_ok())
    }
    #[test]
    fn context_compare_fail() {
        let result = TypingContext {
            span: Span::default(),
            bindings: vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_int(),
            }],
        }
        .compare_to(&TypingContext {
            span: Span::default(),
            bindings: vec![ContextBinding::TypedCovar {
                covar: "a".to_owned(),
                ty: Ty::mk_int(),
            }],
        });
        assert!(result.is_err())
    }

    // Checking variable and covariable lookup
    //
    //

    #[test]
    fn var_lookup() {
        assert!(example_context().lookup_var(&"x".to_owned()).is_ok())
    }

    #[test]
    fn var_lookup_fail() {
        assert!(example_context().lookup_var(&"z".to_owned()).is_err())
    }

    #[test]
    fn covar_lookup() {
        assert!(example_context().lookup_covar(&"a".to_owned()).is_ok())
    }

    #[test]
    fn covar_lookup_fail() {
        assert!(example_context().lookup_covar(&"b".to_owned()).is_err())
    }
}
