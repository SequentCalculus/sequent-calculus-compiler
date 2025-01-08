use codespan::Span;
use derivative::Derivative;
use miette::SourceSpan;
use printer::{
    tokens::{CNT, COLON},
    DocAllocator, Print,
};

use crate::{
    parser::util::ToMiette,
    syntax::{
        types::{OptTyped, Ty},
        Name, Variable, Covariable
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
            ContextBinding::TypedVar { ty, .. } => Some(ty.clone()),
            ContextBinding::TypedCovar { ty, .. } => Some(ty.clone()),
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
                .text(covar)
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
#[derive(Derivative, Default, Debug, Clone)]
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
                    if vars.contains(covar) {
                        return Err(Error::CovarBoundMultipleTimes {
                            span: self.span.to_miette(),
                            covar: covar.clone(),
                            name: binding_site,
                        });
                    }
                    vars.insert(covar.clone());
                }
            }
        }
        Ok(())
    }

    /// Look up the type of a variable in the context.
    pub fn lookup_var(&self, searched_var: &Variable, span: &SourceSpan) -> Result<Ty, Error> {
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
                ContextBinding::TypedCovar { covar, .. } => {
                    if covar == searched_var {
                        return Err(Error::ExpectedTermGotCovariable { span: *span });
                    }
                    continue;
                }
            }
        }
        Err(Error::UnboundVariable {
            span: *span,
            var: searched_var.clone(),
        })
    }

    /// Look up the type of a covariable in the context.
    pub fn lookup_covar(&self, searched_covar: &Covariable, span: &SourceSpan) -> Result<Ty, Error> {
        // Due to variable shadowing we have to traverse from
        // right to left.
        for binding in self.bindings.iter().rev() {
            match binding {
                ContextBinding::TypedVar { var, .. } => {
                    if var == searched_covar {
                        return Err(Error::ExpectedCovariableGotTerm { span: *span });
                    }
                    continue;
                }
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

    pub fn add_var(&mut self, v: &str, ty: Ty) {
        self.bindings.push(ContextBinding::TypedVar {
            var: v.to_owned(),
            ty,
        });
    }

    pub fn add_covar(&mut self, cv: &str, ty: Ty) {
        self.bindings.push(ContextBinding::TypedCovar {
            covar: cv.to_owned(),
            ty,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::util::ToMiette,
        syntax::{context::TypingContext, types::Ty},
        test_common::symbol_table_list,
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use printer::Print;

    /// The context:
    /// `x: Int, y: ListInt, a :cnt Int`
    fn example_context() -> TypingContext {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        ctx.add_var("y", Ty::mk_decl("ListInt"));
        ctx.add_covar("a", Ty::mk_i64());
        ctx
    }

    fn example_context_dup() -> TypingContext {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        ctx.add_covar("a", Ty::mk_i64());
        ctx.add_var("x", Ty::mk_i64());
        ctx
    }

    // Checking prettyprinting
    //
    //

    #[test]
    fn print_context() {
        assert_eq!(
            example_context().print_to_string(None),
            "(x: i64, y: ListInt, a :cnt i64)"
        )
    }

    #[test]
    fn print_context_empty() {
        assert_eq!(TypingContext::default().print_to_string(None), "")
    }

    // Checking well-formedness of contexts
    //
    //

    #[test]
    fn context_check() {
        let symbol_table = symbol_table_list();
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
        let mut ctx1 = TypingContext::default();
        ctx1.add_var("x", Ty::mk_i64());
        let mut ctx2 = TypingContext::default();
        ctx2.add_var("y", Ty::mk_i64());
        let result = ctx1.compare_to(&ctx2);
        assert!(result.is_ok())
    }

    #[test]
    fn context_compare_fail() {
        let mut ctx1 = TypingContext::default();
        ctx1.add_var("x", Ty::mk_i64());
        let mut ctx2 = TypingContext::default();
        ctx2.add_covar("a", Ty::mk_i64());
        let result = ctx1.compare_to(&ctx2);
        assert!(result.is_err())
    }

    // Checking variable and covariable lookup
    //
    //

    #[test]
    fn var_lookup() {
        assert!(example_context()
            .lookup_var(&"x".to_owned(), &Span::default().to_miette())
            .is_ok())
    }

    #[test]
    fn var_lookup_fail() {
        assert!(example_context()
            .lookup_var(&"z".to_owned(), &Span::default().to_miette())
            .is_err())
    }

    #[test]
    fn covar_lookup() {
        assert!(example_context()
            .lookup_covar(&"a".to_owned(), &Span::default().to_miette())
            .is_ok())
    }

    #[test]
    fn covar_lookup_fail() {
        assert!(example_context()
            .lookup_covar(&"b".to_owned(), &Span::default().to_miette())
            .is_err())
    }
}
