use printer::{theme::ThemeExt, tokens::CASE, DocAllocator, Print};

use super::FsTerm;
use crate::{
    syntax_var::clause::print_clauses,
    syntax_var::{FsClause, Var},
    traits::substitution::SubstVar,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsXCase {
    pub clauses: Vec<FsClause>,
}

impl Print for FsXCase {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(CASE)
            .append(alloc.space())
            .append(print_clauses(&self.clauses, cfg, alloc))
    }
}

impl From<FsXCase> for FsTerm {
    fn from(value: FsXCase) -> Self {
        FsTerm::XCase(value)
    }
}

impl SubstVar for FsXCase {
    type Target = FsXCase;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        FsXCase {
            clauses: self.clauses.subst_sim(subst),
        }
    }
}
