use printer::{tokens::COLON, DocAllocator, Print};

use crate::{
    syntax::{Chirality, Ty},
    traits::substitution::SubstVar,
};

use super::Var;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FsContextBinding {
    pub var: Var,
    pub chi: Chirality,
    pub ty: Ty,
}

pub type FsTypingContext = Vec<FsContextBinding>;

impl Print for FsContextBinding {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(&self.var)
            .append(alloc.space())
            .append(alloc.text(COLON))
            .append(self.chi.print(cfg, alloc))
            .append(alloc.space())
            .append(self.ty.print(cfg, alloc))
    }
}

impl SubstVar for FsContextBinding {
    type Target = FsContextBinding;

    fn subst_sim(self, subst: &[(Var, Var)]) -> FsContextBinding {
        FsContextBinding {
            var: self.var.subst_sim(subst),
            ..self
        }
    }
}

#[must_use]
pub fn context_vars(context: &FsTypingContext) -> Vec<Var> {
    let mut vars = Vec::with_capacity(context.len());
    for binding in context {
        vars.push(binding.var.clone());
    }
    vars
}
