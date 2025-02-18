use printer::tokens::{COMMA, FAT_ARROW};
use printer::util::BracesExt;
use printer::{DocAllocator, Print};

use crate::syntax::{Name, Statement, TypingContext, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub xtor: Name,
    pub context: TypingContext,
    pub body: Rc<Statement>,
}

impl FreeVars for Clause {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.body.free_vars(vars);
        for binding in &self.context.bindings {
            vars.remove(&binding.var);
        }
    }
}

impl Subst for Clause {
    type Target = Clause;

    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Clause {
        self.body = self.body.subst_sim(subst);
        self
    }
}

impl Print for Clause {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(&self.xtor)
            .append(self.context.print(cfg, alloc))
            .append(alloc.space())
            .append(FAT_ARROW)
            .append(alloc.line().nest(cfg.indent))
            .append(self.body.print(cfg, alloc).nest(cfg.indent))
    }
}

pub fn print_clauses<'a>(
    clauses: &'a [Clause],
    cfg: &printer::PrintCfg,
    alloc: &'a printer::Alloc<'a>,
) -> printer::Builder<'a> {
    match clauses.len() {
        0 => alloc.space().braces_anno(),
        1 => alloc
            .line()
            .append(clauses[0].print(cfg, alloc))
            .nest(cfg.indent)
            .append(alloc.line())
            .braces_anno()
            .group(),
        _ => {
            let sep = alloc.text(COMMA).append(alloc.hardline());
            alloc
                .hardline()
                .append(alloc.intersperse(clauses.iter().map(|x| x.print(cfg, alloc)), sep.clone()))
                .nest(cfg.indent)
                .append(alloc.hardline())
                .braces_anno()
        }
    }
}
