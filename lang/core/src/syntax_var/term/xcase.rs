use printer::{theme::ThemeExt, tokens::CASE, util::BracesExt, DocAllocator, Print};

use super::Term;
use crate::{
    syntax_var::{Clause, Var},
    traits::{substitution::SubstVar, used_binders::UsedBinders},
};

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XCase {
    pub clauses: Vec<Clause>,
}

impl Print for XCase {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword(CASE).append(alloc.space()).append(
            alloc
                .space()
                .append(self.clauses.print(cfg, alloc))
                .append(alloc.space())
                .braces_anno(),
        )
    }
}

impl From<XCase> for Term {
    fn from(value: XCase) -> Self {
        Term::XCase(value)
    }
}

impl UsedBinders for XCase {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.clauses.used_binders(used);
    }
}

impl SubstVar for XCase {
    type Target = XCase;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        XCase {
            clauses: self.clauses.subst_sim(subst),
        }
    }
}
