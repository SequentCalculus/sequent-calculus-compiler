use printer::tokens::COLON;
use printer::{DocAllocator, Print};

use super::{Chirality, Ty, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::substitution::Subst;

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct ContextBinding {
    pub var: Var,
    pub chi: Chirality,
    pub ty: Ty,
}

pub type TypingContext = Vec<ContextBinding>;

impl Print for ContextBinding {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(&self.var)
            .append(alloc.space())
            .append(COLON)
            .append(self.chi.print(cfg, alloc))
            .append(COLON)
            .append(alloc.space())
            .append(self.ty.print(cfg, alloc))
    }
}

impl FreeVars for ContextBinding {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        vars.insert(self.var.clone());
    }
}

impl Subst for ContextBinding {
    type Target = ContextBinding;

    fn subst_sim(self, subst: &[(Var, Var)]) -> ContextBinding {
        ContextBinding {
            var: self.var.subst_sim(subst),
            ..self
        }
    }
}

#[must_use]
pub fn context_vars(context: &TypingContext) -> Vec<Var> {
    let mut vars = Vec::with_capacity(context.len());
    for binding in context {
        vars.push(binding.var.clone());
    }
    vars
}

#[must_use]
pub fn lookup_variable_context<'a>(var: &str, context: &'a [ContextBinding]) -> &'a ContextBinding {
    let context_binding = context
        .iter()
        .find(|binding| var == binding.var)
        .expect("Variable {var} not found in context {context:?}");
    context_binding
}

#[cfg(test)]
mod context_tests {
    use super::{context_vars, lookup_variable_context, ContextBinding, FreeVars, Subst};
    use crate::syntax::{types::Ty, Chirality};
    use printer::Print;
    use std::collections::HashSet;

    fn example_binding() -> ContextBinding {
        ContextBinding {
            var: "x".to_owned(),
            chi: Chirality::Prd,
            ty: Ty::Int,
        }
    }

    #[test]
    fn print_binding() {
        let result = example_binding().print_to_string(Default::default());
        let expected = "x :prd: Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_binding() {
        let mut result = HashSet::new();
        example_binding().free_vars(&mut result);
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_binding() {
        let result = example_binding().subst_sim(&vec![("x".to_owned(), "y".to_owned())]);
        let expected = ContextBinding {
            var: "y".to_owned(),
            chi: Chirality::Prd,
            ty: Ty::Int,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn binding_vars() {
        let result = context_vars(&vec![example_binding()]);
        let expected = vec!["x".to_owned()];
        assert_eq!(result, expected)
    }

    #[test]
    fn lookup_binding() {
        let bindings = vec![example_binding()];
        let result = lookup_variable_context("x", &bindings);
        let expected = example_binding();
        assert_eq!(result, &expected)
    }
}
