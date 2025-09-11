//! This module defines explicit substitutions in AxCut.

use printer::theme::ThemeExt;
use printer::tokens::{COMMA, SEMI, SUBSTITUTE};
use printer::{DocAllocator, Print};

use super::{Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

/// This module defines explicit substitutions in AxCut. They consist of a list of assignments of
/// new variables to old variables according to which the context is rearranged, and the remaining
/// statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Substitute {
    pub rearrange: Vec<(Var, Var)>,
    pub next: Rc<Statement>,
}

impl Print for Substitute {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let sep = alloc.text(COMMA).append(alloc.line());
        let rearrange = alloc.intersperse(
            self.rearrange.iter().map(|(new, old)| {
                new.print(cfg, alloc)
                    .append(" := ")
                    .append(old.print(cfg, alloc))
                    .parens()
            }),
            sep,
        );
        alloc
            .keyword(SUBSTITUTE)
            .append(alloc.line())
            .append(rearrange)
            .nest(cfg.indent)
            .group()
            .append(SEMI)
            .append(alloc.hardline())
            .append(self.next.print(cfg, alloc).group())
    }
}

impl From<Substitute> for Statement {
    fn from(value: Substitute) -> Self {
        Statement::Substitute(value)
    }
}

impl FreeVars for Substitute {
    fn free_vars(mut self, vars: &mut HashSet<Var>) -> Self {
        self.next = self.next.free_vars(vars);

        for (new, old) in &self.rearrange {
            vars.insert(old.clone());
            vars.remove(new);
        }

        self
    }
}

impl Subst for Substitute {
    // this function is actually never called on the linearized version of AxCut containing
    // explicit substitutions
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Substitute {
        self.rearrange = self
            .rearrange
            .into_iter()
            .map(|(new, old)| (new.subst_sim(subst), old.subst_sim(subst)))
            .collect();
        self.next = self.next.subst_sim(subst);
        self
    }
}
