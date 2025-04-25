use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{CNS, COLON, PRD},
};

use super::{Ty, Var};
use crate::traits::*;

use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
            Chirality::Prd => alloc.keyword(PRD),
            Chirality::Cns => alloc.keyword(CNS),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct ContextBinding {
    pub var: Var,
    pub chi: Chirality,
    pub ty: Ty,
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

impl SubstVar for ContextBinding {
    type Target = ContextBinding;
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> ContextBinding {
        self.var = self.var.subst_sim(subst);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TypingContext {
    pub bindings: Vec<ContextBinding>,
}

impl TypingContext {
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

    pub fn vars(&self) -> HashSet<Var> {
        self.bindings
            .iter()
            .map(|binding| binding.var.clone())
            .collect()
    }

    pub fn vec_vars(&self) -> Vec<Var> {
        let mut vars = Vec::with_capacity(self.bindings.len());
        for binding in &self.bindings {
            vars.push(binding.var.clone());
        }
        vars
    }
}

impl From<VecDeque<ContextBinding>> for TypingContext {
    fn from(bindings: VecDeque<ContextBinding>) -> TypingContext {
        TypingContext {
            bindings: bindings.into_iter().collect(),
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

impl SubstVar for TypingContext {
    type Target = TypingContext;
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> TypingContext {
        self.bindings = self.bindings.subst_sim(subst);
        self
    }
}
