//! This module defines declaration-site type parameters, i.e. the type parameters introduced by
//! a `data`/`codata`/`def` declaration or by a constructor's/destructor's own existential
//! parameters. Unlike [`TypeContext`](crate::syntax::context::TypeContext) (used for
//! pattern-binding occurrences in `case`/`new` clauses, where the polarity is inherited from the
//! declaration and not re-annotated), every binding here carries its own declared [`Polarity`].

use derivative::Derivative;
use miette::SourceSpan;
use printer::*;

use crate::parser::util::ToMiette;
use crate::syntax::context::TypeContext;
use crate::syntax::declarations::Polarity;
use crate::syntax::names::{Name, Var};
use crate::typing::errors::Error;

use std::collections::HashSet;

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

/// This struct defines a declaration-site list of [`TypeParam`]s.
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

#[cfg(test)]
mod tests {
    use super::{TypeParam, TypeParams};
    use crate::syntax::declarations::Polarity;
    use printer::Print;

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
}
