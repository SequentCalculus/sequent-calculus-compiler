//! This module defines the exit statement in AxCut.

use printer::{DocAllocator, Print, theme::ThemeExt, tokens::EXIT};

use super::{Statement, Var};
use crate::traits::substitution::Subst;

/// This struct defines the exit statement in AxCut. It consists of a variable which contains the
/// exit code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exit {
    pub var: Var,
}

impl Print for Exit {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword(EXIT).append(alloc.space()).append(&self.var)
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
