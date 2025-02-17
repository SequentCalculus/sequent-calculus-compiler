use printer::theme::ThemeExt;
use printer::tokens::{COMMA, SEMI, SUBSTITUTE};
use printer::{DocAllocator, Print};

use super::{Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

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
        let rearrange = alloc.intersperse(
            self.rearrange
                .iter()
                .map(|(new, old)| alloc.text(new).append(" !-> ").append(old).parens()),
            alloc.text(COMMA).append(alloc.space()),
        );
        alloc
            .keyword(SUBSTITUTE)
            .append(alloc.space())
            .append(rearrange)
            .append(SEMI)
            .append(alloc.line())
            .append(self.next.print(cfg, alloc))
    }
}

impl From<Substitute> for Statement {
    fn from(value: Substitute) -> Self {
        Statement::Substitute(value)
    }
}

impl FreeVars for Substitute {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.next.free_vars(vars);
        for (new, old) in &self.rearrange {
            vars.insert(old.clone());
            vars.remove(new);
        }
    }
}

impl Subst for Substitute {
    type Target = Substitute;

    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Substitute {
        self.rearrange = self
            .rearrange
            .into_iter()
            .map(|(new, old)| (new, old.subst_sim(subst)))
            .collect();
        self.next = self.next.subst_sim(subst);
        self
    }
}
