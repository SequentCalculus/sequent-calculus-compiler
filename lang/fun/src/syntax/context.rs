//! This module defines typing contexts in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{CNS, COLON};
use printer::*;

use crate::parser::util::ToMiette;
use crate::syntax::*;
use crate::traits::*;
use crate::typing::*;

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
    fn print<'a>(&'a self, _cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
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
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
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
    pub span: Option<SourceSpan>,
    /// The context bindings
    pub bindings: Vec<ContextBinding>,
}

impl TypingContext {
    pub fn vars(&self) -> HashSet<Var> {
        self.bindings.iter().map(|bind| bind.var.clone()).collect()
    }

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
                .check_template(self.span, symbol_table, type_params)?;
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
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let sep = if cfg.allow_linebreaks {
            alloc.line_()
        } else {
            alloc.nil()
        };

        if self.bindings.is_empty() {
            alloc.nil()
        } else {
            sep.clone()
                .append(self.bindings.print(cfg, alloc))
                .nest(cfg.indent)
                .append(sep)
        }
    }
}

/// This struct defines name context, which is a list of parameters without types.
#[derive(Derivative, Default, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct NameContext {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Option<SourceSpan>,
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
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let sep = if cfg.allow_linebreaks {
            alloc.line_()
        } else {
            alloc.nil()
        };

        if self.bindings.is_empty() {
            alloc.nil()
        } else {
            sep.clone()
                .append(self.bindings.print(cfg, alloc))
                .nest(cfg.indent)
                .append(sep)
                .parens()
                .group()
        }
    }
}

/// This struct defines a type context, which is a list of type parameters.
#[derive(Derivative, Default, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct TypeContext {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Option<SourceSpan>,
    /// The type bindings
    pub bindings: Vec<Name>,
}

/// This struct defines a single type parameter at a declaration site (`data`/`codata`/`def`, or a
/// constructor's/destructor's own existential parameters). Every such type parameter must carry an
/// explicit, mandatory [`Polarity`] annotation (`A+` for data/positive, `A-` for codata/negative) —
/// there is no default and no inference.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct TypeParam {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Option<SourceSpan>,
    /// The parameter name
    pub name: Name,
    /// The declared polarity of the parameter
    pub polarity: Polarity,
}

/// This struct defines a declaration-site list of [`TypeParam`]s, i.e. the type parameters
/// introduced by a `data`/`codata`/`def` declaration or by a constructor's/destructor's own
/// existential parameters. Unlike [`TypeContext`] (used for pattern-binding occurrences in `case`/
/// `new` clauses, where the polarity is inherited from the declaration and not re-annotated), every
/// binding here carries its own declared [`Polarity`].
#[derive(Derivative, Default, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct TypeParams {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Option<SourceSpan>,
    /// The type parameter bindings
    pub bindings: Vec<TypeParam>,
}

impl TypeParams {
    /// This function checks that no variable in the type parameter list is duplicated.
    /// - `binding_site` is the name of the definition where the check was triggered.
    pub fn no_dups(&self, binding_site: &str) -> Result<(), Error> {
        let mut params: HashSet<Var> = HashSet::new();
        for binding in &self.bindings {
            if params.contains(&binding.name) {
                return Err(Error::TypeParameterBoundMultipleTimes {
                    span: self.span.to_miette(),
                    param: binding.name.clone(),
                    name: binding_site.to_string(),
                });
            }
            params.insert(binding.name.clone());
        }
        Ok(())
    }

    /// This function constructs a type parameter list with empty source location from a list of
    /// `(name, polarity)` pairs.
    pub fn mk(params: &[(&str, Polarity)]) -> TypeParams {
        TypeParams {
            span: None,
            bindings: params
                .iter()
                .map(|(name, polarity)| TypeParam {
                    span: None,
                    name: name.to_string(),
                    polarity: polarity.clone(),
                })
                .collect(),
        }
    }

    /// This function projects the parameter names, discarding polarity.
    pub fn names(&self) -> Vec<Name> {
        self.bindings
            .iter()
            .map(|param| param.name.clone())
            .collect()
    }

    /// This function projects this list down to a bare [`TypeContext`] (names only, discarding
    /// polarity) for use by code that only needs to check name scoping, not polarity.
    pub fn to_type_context(&self) -> TypeContext {
        TypeContext {
            span: self.span,
            bindings: self.names(),
        }
    }
}

impl Print for TypeParam {
    fn print<'a>(&'a self, _cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let sigil = match self.polarity {
            Polarity::Data => "+",
            Polarity::Codata => "-",
        };
        alloc.text(self.name.clone()).append(alloc.text(sigil))
    }
}

impl Print for TypeParams {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let sep = if cfg.allow_linebreaks {
            alloc.line_()
        } else {
            alloc.nil()
        };

        if self.bindings.is_empty() {
            alloc.nil()
        } else {
            sep.clone()
                .append(self.bindings.print(cfg, alloc))
                .nest(cfg.indent)
                .append(sep)
                .brackets()
                .group()
        }
    }
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
            span: None,
            bindings: params.iter().map(ToString::to_string).collect(),
        }
    }

    /// This function extends a type context with another type context.
    pub fn extend(&self, other: TypeContext) -> TypeContext {
        let mut extended = self.clone();
        extended.bindings.extend(other.bindings);
        extended
    }
}

impl Print for TypeContext {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let sep = if cfg.allow_linebreaks {
            alloc.line_()
        } else {
            alloc.nil()
        };

        if self.bindings.is_empty() {
            alloc.nil()
        } else {
            sep.clone()
                .append(self.bindings.print(cfg, alloc))
                .nest(cfg.indent)
                .append(sep)
                .brackets()
                .group()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        syntax::{
            context::TypingContext,
            types::{Ty, TypeArgs},
            util::dummy_span,
        },
        test_common::symbol_table_list,
        typing::symbol_table::SymbolTable,
    };
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
            "x: i64, y: List[i64], a: cns i64"
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

    use crate::syntax::declarations::Polarity;
    use crate::syntax::{TypeParam, TypeParams};

    #[test]
    fn type_params_no_dups_ok() {
        let params = TypeParams::mk(&[("A", Polarity::Data), ("B", Polarity::Codata)]);
        assert!(params.no_dups("binding site").is_ok())
    }

    #[test]
    fn type_params_no_dups_fail_same_polarity() {
        let params = TypeParams::mk(&[("A", Polarity::Data), ("A", Polarity::Data)]);
        assert!(params.no_dups("binding site").is_err())
    }

    #[test]
    fn type_params_no_dups_fail_different_polarity() {
        // Duplicated name, even with different declared polarities, is still a duplication error.
        let params = TypeParams::mk(&[("A", Polarity::Data), ("A", Polarity::Codata)]);
        assert!(params.no_dups("binding site").is_err())
    }

    #[test]
    fn type_params_names_projects_bare_names() {
        let params = TypeParams::mk(&[("A", Polarity::Data), ("B", Polarity::Codata)]);
        assert_eq!(params.names(), vec!["A".to_string(), "B".to_string()]);
    }

    #[test]
    fn type_params_print() {
        let params = TypeParams::mk(&[("A", Polarity::Data), ("B", Polarity::Codata)]);
        assert_eq!(params.print_to_string(None), "[A+, B-]");
    }

    #[test]
    fn type_param_print() {
        let plus = TypeParam {
            span: None,
            name: "A".to_string(),
            polarity: Polarity::Data,
        };
        let minus = TypeParam {
            span: None,
            name: "B".to_string(),
            polarity: Polarity::Codata,
        };
        assert_eq!(plus.print_to_string(None), "A+");
        assert_eq!(minus.print_to_string(None), "B-");
    }

    // Checking variable and covariable lookup
    //
    //

    #[test]
    fn var_lookup() {
        assert!(
            example_context()
                .lookup_var(&"x".to_owned(), &dummy_span())
                .is_ok()
        )
    }

    #[test]
    fn var_lookup_fail() {
        assert!(
            example_context()
                .lookup_var(&"z".to_owned(), &dummy_span())
                .is_err()
        )
    }

    #[test]
    fn covar_lookup() {
        assert!(
            example_context()
                .lookup_covar(&"a".to_owned(), &dummy_span())
                .is_ok()
        )
    }

    #[test]
    fn covar_lookup_fail() {
        assert!(
            example_context()
                .lookup_covar(&"b".to_owned(), &dummy_span())
                .is_err()
        )
    }
}
