//! This module defines the exit statement in AxCut.

use printer::{DocAllocator, Print, theme::ThemeExt, tokens::EXIT};

use crate::syntax::{Chirality, ContextBinding, Statement, Ty, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::substitution::Subst;
use crate::traits::typed_free_vars::TypedFreeVars;

use std::collections::{BTreeSet, HashSet};

/// This struct defines the exit statement in AxCut. It consists of a variable which contains the
/// exit code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exit {
    pub var: Var,
}

impl Print for Exit {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(EXIT)
            .append(alloc.space())
            .append(self.var.print(cfg, alloc))
    }
}

impl From<Exit> for Statement {
    fn from(value: Exit) -> Self {
        Statement::Exit(value)
    }
}

impl Subst for Exit {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Exit {
        self.var = self.var.subst_sim(subst);
        self
    }
}

impl FreeVars for Exit {
    fn free_vars(self, vars: &mut HashSet<Var>) -> Self {
        vars.insert(self.var.clone());
        self
    }
}

impl TypedFreeVars for Exit {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        vars.insert(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Ext,
            ty: Ty::I64,
        });
    }
}
