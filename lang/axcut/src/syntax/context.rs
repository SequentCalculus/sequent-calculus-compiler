use printer::theme::ThemeExt;
use printer::tokens::{CNS, COLON, EXT, PRD};
use printer::{DocAllocator, Print};

use super::{Ty, Var};
use crate::traits::substitution::Subst;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Chirality {
    Prd,
    Cns,
    Ext,
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
            Chirality::Ext => alloc.keyword(EXT),
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
            .append(COLON)
            .append(alloc.space())
            .append(self.ty.print(cfg, alloc))
    }
}

impl Subst for ContextBinding {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> ContextBinding {
        self.var = self.var.subst_sim(subst);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypingContext {
    pub bindings: Vec<ContextBinding>,
}

impl TypingContext {
    pub fn vars(&self) -> Vec<Var> {
        let mut vars = Vec::with_capacity(self.bindings.len());
        for binding in &self.bindings {
            vars.push(binding.var.clone());
        }
        vars
    }

    pub fn lookup_variable<'a>(&'a self, var: &str) -> &'a ContextBinding {
        let context_binding = self
            .bindings
            .iter()
            .find(|binding| var == binding.var)
            .unwrap_or_else(|| {
                panic!(
                    "Variable {var} not found in context {}",
                    self.print_to_string(None)
                )
            });
        context_binding
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

impl From<Vec<ContextBinding>> for TypingContext {
    fn from(bindings: Vec<ContextBinding>) -> Self {
        TypingContext { bindings }
    }
}
