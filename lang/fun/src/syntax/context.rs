use codespan::Span;
use derivative::Derivative;
use miette::SourceSpan;
use printer::{
    theme::ThemeExt,
    tokens::{CNS, COLON, COMMA},
    DocAllocator, Print,
};

use crate::{
    parser::util::ToMiette,
    syntax::{
        types::{OptTyped, Ty},
        Covar, Name, Var,
    },
    typing::{errors::Error, symbol_table::SymbolTable},
};

use std::collections::{HashMap, HashSet};

// Context Bindings
//
//

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub enum Chirality {
    Prd,
    Cns,
}

impl Print for Chirality {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Chirality::Prd => alloc.nil(),
            Chirality::Cns => alloc.keyword(CNS),
        }
    }
}

/// Describes a single binding that can occur in a typing context.
/// Either
/// - A variable binding: `x : ty`
/// - A covariable binding `'a :cns ty`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextBinding {
    pub var: Var,
    pub chi: Chirality,
    pub ty: Ty,
}

impl ContextBinding {
    pub fn subst_ty(self, mappings: &HashMap<Name, Ty>) -> ContextBinding {
        ContextBinding {
            ty: self.ty.subst_ty(mappings),
            ..self
        }
    }
}

impl Print for ContextBinding {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.var
            .print(cfg, alloc)
            .append(alloc.space())
            .append(COLON)
            .append(self.chi.print(cfg, alloc))
            .append(alloc.space())
            .append(self.ty.print(cfg, alloc))
    }
}

impl OptTyped for ContextBinding {
    fn get_type(&self) -> Option<Ty> {
        Some(self.ty.clone())
    }
}

// TypingContext
//
//

/// A typing context.
/// Example:
/// `x : Int, y : ListInt`
#[derive(Derivative, Default, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct TypingContext {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub bindings: Vec<ContextBinding>,
}

impl TypingContext {
    /// Check whether all types in the typing context are valid.
    pub fn check(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        for binding in &self.bindings {
            binding.ty.check(&self.span, symbol_table)?;
        }
        Ok(())
    }

    /// Check whether all types in the typing context of a template are valid.
    pub fn check_template(
        &self,
        symbol_table: &SymbolTable,
        type_params: &TypeContext,
    ) -> Result<(), Error> {
        for binding in &self.bindings {
            binding
                .ty
                .check_template(&self.span, symbol_table, type_params)?;
        }
        Ok(())
    }

    /// Check whether no variable in the typing context is duplicated.
    pub fn no_dups(&self, binding_site: &str) -> Result<(), Error> {
        let mut vars: HashSet<Var> = HashSet::new();
        for binding in &self.bindings {
            if vars.contains(&binding.var) {
                if binding.chi == Chirality::Prd {
                    return Err(Error::VarBoundMultipleTimes {
                        span: self.span.to_miette(),
                        var: binding.var.clone(),
                        name: binding_site.to_string(),
                    });
                }
                return Err(Error::CovarBoundMultipleTimes {
                    span: self.span.to_miette(),
                    covar: binding.var.clone(),
                    name: binding_site.to_string(),
                });
            }
            vars.insert(binding.var.clone());
        }
        Ok(())
    }

    /// Look up the type of a variable in the context.
    pub fn lookup_var(&self, searched_var: &Var, span: &SourceSpan) -> Result<Ty, Error> {
        // Due to variable shadowing we have to traverse from
        // right to left.
        for binding in self.bindings.iter().rev() {
            if binding.var == *searched_var {
                if binding.chi == Chirality::Cns {
                    return Err(Error::ExpectedTermGotCovariable { span: *span });
                }
                return Ok(binding.ty.clone());
            }
        }
        Err(Error::UnboundVariable {
            span: *span,
            var: searched_var.clone(),
        })
    }

    /// Look up the type of a covariable in the context.
    pub fn lookup_covar(&self, searched_covar: &Covar, span: &SourceSpan) -> Result<Ty, Error> {
        // Due to variable shadowing we have to traverse from
        // right to left.
        for binding in self.bindings.iter().rev() {
            if binding.var == *searched_covar {
                if binding.chi == Chirality::Prd {
                    return Err(Error::ExpectedCovariableGotTerm { span: *span });
                }
                return Ok(binding.ty.clone());
            }
        }
        Err(Error::UnboundCovariable {
            span: *span,
            covar: searched_covar.clone(),
        })
    }

    pub fn add_var(&mut self, var: &str, ty: Ty) {
        self.bindings.push(ContextBinding {
            var: var.to_owned(),
            chi: Chirality::Prd,
            ty,
        });
    }

    pub fn add_covar(&mut self, covar: &str, ty: Ty) {
        self.bindings.push(ContextBinding {
            var: covar.to_owned(),
            chi: Chirality::Cns,
            ty,
        });
    }

    pub fn subst_ty(self, mappings: &HashMap<Name, Ty>) -> TypingContext {
        TypingContext {
            bindings: self
                .bindings
                .into_iter()
                .map(|binding| binding.subst_ty(mappings))
                .collect(),
            ..self
        }
    }
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

// NameContext
//
//

/// A list of parameters without types.
#[derive(Derivative, Default, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct NameContext {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub bindings: Vec<Name>,
}

impl Print for NameContext {
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

impl NameContext {
    /// Check whether no variable in the context is duplicated.
    pub fn no_dups(&self, binding_site: &str) -> Result<(), Error> {
        let mut params: HashSet<Var> = HashSet::new();
        for binding in &self.bindings {
            if params.contains(binding) {
                return Err(Error::TypeParameterBoundMultipleTimes {
                    span: self.span.to_miette(),
                    param: binding.clone(),
                    name: binding_site.to_string(),
                });
            }
            params.insert(binding.clone());
        }
        Ok(())
    }

    /// Add types for the variables in a name context according to a given typing context.
    pub fn add_types(&self, expected: &TypingContext) -> Result<TypingContext, Error> {
        if self.bindings.len() != expected.bindings.len() {
            return Err(Error::WrongNumberOfBinders {
                span: self.span.to_miette(),
                expected: expected.bindings.len(),
                provided: self.bindings.len(),
            });
        }
        let mut context_with_types = TypingContext {
            span: self.span,
            bindings: Vec::new(),
        };
        for (name, binding) in self.bindings.iter().zip(expected.bindings.iter()) {
            context_with_types.bindings.push(ContextBinding {
                var: name.clone(),
                ..binding.clone()
            });
        }
        Ok(context_with_types)
    }
}

// TypeContext
//
//

/// A list of type parameters.
#[derive(Derivative, Default, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct TypeContext {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub bindings: Vec<Name>,
}

impl TypeContext {
    /// Check whether no variable in the type context is duplicated.
    pub fn no_dups(&self, binding_site: &str) -> Result<(), Error> {
        let mut params: HashSet<Var> = HashSet::new();
        for binding in &self.bindings {
            if params.contains(binding) {
                return Err(Error::TypeParameterBoundMultipleTimes {
                    span: self.span.to_miette(),
                    param: binding.clone(),
                    name: binding_site.to_string(),
                });
            }
            params.insert(binding.clone());
        }
        Ok(())
    }

    pub fn mk(params: &[&str]) -> TypeContext {
        TypeContext {
            span: Span::default(),
            bindings: params.iter().map(ToString::to_string).collect(),
        }
    }
}

impl Print for TypeContext {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.bindings.is_empty() {
            alloc.nil()
        } else {
            let sep = alloc.text(COMMA).append(alloc.space());
            alloc
                .intersperse(self.bindings.iter().map(|binding| alloc.typ(binding)), sep)
                .brackets()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::util::ToMiette,
        syntax::{
            context::TypingContext,
            types::{Ty, TypeArgs},
        },
        test_common::symbol_table_list,
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use printer::Print;

    /// The context:
    /// `x : i64, y : List[i64], a :cns i64`
    fn example_context() -> TypingContext {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        ctx.add_var("y", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
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
            "(x : i64, y : List[i64], a :cns i64)"
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
        let mut symbol_table = symbol_table_list();
        assert!(example_context().check(&mut symbol_table).is_ok())
    }
    #[test]
    fn context_check_fail() {
        assert!(example_context()
            .check(&mut SymbolTable::default())
            .is_err())
    }
    #[test]
    fn context_check_fail_dup() {
        assert!(example_context_dup().no_dups("binding site").is_err())
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
