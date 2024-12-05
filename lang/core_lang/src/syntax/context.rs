use printer::{
    tokens::{COLON, TICK},
    DocAllocator, Print,
};

use super::{Covar, Ty, Var};
use crate::traits::*;

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context<T> {
    pub bindings: Vec<T>,
}

impl<T: Print> Print for Context<T> {
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

impl<T: Focusing> Focusing for Context<T> {
    type Target = Context<T::Target>;

    fn focus(self, state: &mut FocusingState) -> Self::Target {
        Context {
            bindings: self.bindings.focus(state),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ContextBinding {
    VarBinding { var: Var, ty: Ty },
    CovarBinding { covar: Covar, ty: Ty },
}

pub type TypingContext = Context<ContextBinding>;

impl Print for ContextBinding {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            ContextBinding::VarBinding { var, ty } => alloc
                .text(var)
                .append(alloc.text(COLON))
                .append(alloc.space())
                .append(ty.print(cfg, alloc)),
            ContextBinding::CovarBinding { covar, ty } => alloc
                .text(TICK)
                .append(covar.print(cfg, alloc))
                .append(alloc.space())
                .append(alloc.text(":cns"))
                .append(alloc.space())
                .append(ty.print(cfg, alloc)),
        }
    }
}

impl TypingContext {
    #[must_use]
    pub fn vars(&self) -> HashSet<Var> {
        self.bindings
            .iter()
            .filter_map(|bnd| match bnd {
                ContextBinding::VarBinding { var, ty: _ } => Some(var.clone()),
                ContextBinding::CovarBinding { .. } => None,
            })
            .collect()
    }

    #[must_use]
    pub fn covars(&self) -> HashSet<Covar> {
        self.bindings
            .iter()
            .filter_map(|bnd| match bnd {
                ContextBinding::CovarBinding { covar, ty: _ } => Some(covar.clone()),
                ContextBinding::VarBinding { .. } => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod context_tests {
    use printer::Print;

    use super::{ContextBinding, Ty};

    #[test]
    fn display_var() {
        let result = ContextBinding::VarBinding {
            var: "x".to_owned(),
            ty: Ty::Int,
        }
        .print_to_string(None);
        let expected = "x: Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_covar() {
        let result = ContextBinding::CovarBinding {
            covar: "a".to_owned(),
            ty: Ty::Int,
        }
        .print_to_string(None);
        let expected = "'a :cns Int";
        assert_eq!(result, expected)
    }
}
