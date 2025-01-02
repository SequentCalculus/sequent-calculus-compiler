use printer::{
    tokens::{COLON, TICK},
    DocAllocator, Print,
};

use super::{Covar, Ty, Var};
use crate::traits::*;

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Context<T> {
    pub bindings: Vec<T>,
}

impl<T> Context<T> {
    pub fn new() -> Context<T> {
        Context { bindings: vec![] }
    }
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ContextBinding {
    VarBinding { var: Var, ty: Ty },
    CovarBinding { covar: Covar, ty: Ty },
}

pub type TypingContext = Context<ContextBinding>;

impl TypingContext {
    pub fn empty() -> TypingContext {
        Context { bindings: vec![] }
    }

    pub fn add_var(&mut self, var: &str, ty: Ty) {
        self.bindings.push(ContextBinding::VarBinding {
            var: var.to_owned(),
            ty,
        })
    }

    pub fn add_covar(&mut self, covar: &str, ty: Ty) {
        self.bindings.push(ContextBinding::CovarBinding {
            covar: covar.to_owned(),
            ty,
        })
    }
}

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

    #[must_use]
    pub fn vec_vars(&self) -> Vec<Var> {
        let mut vars = Vec::with_capacity(self.bindings.len());
        for binding in self.bindings.iter() {
            match binding {
                ContextBinding::VarBinding { var, ty: _ } => vars.push(var.clone()),
                ContextBinding::CovarBinding { covar, ty: _ } => vars.push(covar.clone()),
            }
        }
        vars
    }
}

impl SubstVar for ContextBinding {
    type Target = ContextBinding;

    fn subst_sim(self, subst: &[(Var, Var)]) -> ContextBinding {
        match self {
            ContextBinding::VarBinding { var, ty } => ContextBinding::VarBinding {
                var: var.subst_sim(subst),
                ty,
            },
            ContextBinding::CovarBinding { covar, ty } => ContextBinding::CovarBinding {
                covar: covar.subst_sim(subst),
                ty,
            },
        }
    }
}
