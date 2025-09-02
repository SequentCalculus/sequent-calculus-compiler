//! This module defines typing contexts in Fun.

use codespan::Span;
use derivative::Derivative;
use miette::SourceSpan;
use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{CNS, COLON, COMMA},
};

use crate::{
    parser::util::ToMiette,
    syntax::{
        names::{Covar, Name, Var},
        types::{OptTyped, Ty},
    },
    typing::{errors::Error, symbol_table::SymbolTable},
};

use std::collections::{HashMap, HashSet};

/// This enum encodes the chirality of a variable in a context, i.e., whether the binding is for a
/// producer or a consumer.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub enum Chirality {
    /// Producer
    Prd,
    /// Consumer
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
            Chirality::Cns => alloc.space().append(alloc.keyword(CNS)),
        }
    }
}

/// This struct defines a binding in a typing context. It consists of a variable, its [`Chirality`]
/// and its [`Ty`]pe. It is hence either
/// - a variable binding: `x: ty` (in Fun we ususally do not use a `prd` annotation)
/// - a covariable binding `a: cns ty`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextBinding {
    /// The bound variable or covariable
    pub var: Var,
    /// Whether the binding is for a producer or consumer (i.e., a variable or covariable)
    pub chi: Chirality,
    /// The type of the binding
    pub ty: Ty,
}

impl ContextBinding {
    /// This function substitutes type parameters with concrete types in the type of the binding.
    /// - `mappings` contains the substitutions to perform.
    pub fn subst_ty(mut self, mappings: &HashMap<Name, Ty>) -> ContextBinding {
        self.ty = self.ty.subst_ty(mappings);
        self
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

/// This struct defines a typing context. It consists of a list of [`ContextBinding`]s.
#[derive(Derivative, Default, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct TypingContext {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The context bindings
    pub bindings: Vec<ContextBinding>,
}

impl TypingContext {
    /// This function checks whether all types in the typing context are well-formed.
    pub fn check(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        for binding in &self.bindings {
            binding.ty.check(&self.span, symbol_table)?;
        }
        Ok(())
    }

    /// This function checks whether all types in the typing context within (an xtor of) a
    /// template are well-formed.
    /// - `symbol_table` is the symbol table during typechecking.
    /// - `type_params` is the list of type parameters of the template.
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

    /// This function checks that no variable in the typing context is duplicated.
    /// - `binding_site` is the name of the definition where the check was triggered.
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

    /// This function looks up the type of a variable in the context.
    pub fn lookup_var(&self, searched_var: &Var, span: &SourceSpan) -> Result<Ty, Error> {
        // Due to variable shadowing we have to traverse from right to left.
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

    /// This function looks up the type of a covariable in the context.
    pub fn lookup_covar(&self, searched_covar: &Covar, span: &SourceSpan) -> Result<Ty, Error> {
        // Due to variable shadowing we have to traverse from right to left.
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

    /// This function adds a variable (producer) to the context.
    pub fn add_var(&mut self, var: &str, ty: Ty) {
        self.bindings.push(ContextBinding {
            var: var.to_owned(),
            chi: Chirality::Prd,
            ty,
        });
    }

    /// This funciton adds a covariable (consumer) to the context.
    pub fn add_covar(&mut self, covar: &str, ty: Ty) {
        self.bindings.push(ContextBinding {
            var: covar.to_owned(),
            chi: Chirality::Cns,
            ty,
        });
    }

    /// This function substitutes type parameters with concrete types in all types found in the
    /// context bindings.
    /// - `mappings` contains the substitutions to perform.
    pub fn subst_ty(mut self, mappings: &HashMap<Name, Ty>) -> TypingContext {
        self.bindings = self
            .bindings
            .into_iter()
            .map(|binding| binding.subst_ty(mappings))
            .collect();
        self
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

/// This struct defines name context, which is a list of parameters without types.
#[derive(Derivative, Default, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct NameContext {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The named bindings
    pub bindings: Vec<Name>,
}

impl NameContext {
    /// This function checks that no variable in the name context is duplicated.
    /// - `binding_site` is the name of the definition where the check was triggered.
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

    /// This function adds types for the variables in the name context according to a given typing
    /// context.
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

/// This struct defines a type context, which is a list of type parameters.
#[derive(Derivative, Default, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct TypeContext {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The type bindings
    pub bindings: Vec<Name>,
}

impl TypeContext {
    /// This function checks that no variable in the type context is duplicated.
    /// - `binding_site` is the name of the definition where the check was triggered.
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

    /// This function constructs a type context with empty source location from a list of strings.
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
    /// `x: i64, y: List[i64], a: cns i64`
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
            "(x: i64, y: List[i64], a: cns i64)"
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
        assert!(
            example_context()
                .check(&mut SymbolTable::default())
                .is_err()
        )
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
        assert!(
            example_context()
                .lookup_var(&"x".to_owned(), &Span::default().to_miette())
                .is_ok()
        )
    }

    #[test]
    fn var_lookup_fail() {
        assert!(
            example_context()
                .lookup_var(&"z".to_owned(), &Span::default().to_miette())
                .is_err()
        )
    }

    #[test]
    fn covar_lookup() {
        assert!(
            example_context()
                .lookup_covar(&"a".to_owned(), &Span::default().to_miette())
                .is_ok()
        )
    }

    #[test]
    fn covar_lookup_fail() {
        assert!(
            example_context()
                .lookup_covar(&"b".to_owned(), &Span::default().to_miette())
                .is_err()
        )
    }
}
