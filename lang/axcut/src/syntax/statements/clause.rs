//! This module defines a clause in a match or a closure in AxCut.

use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{COMMA, FAT_ARROW},
    util::BracesExt,
};

use crate::syntax::{Name, Statement, TypingContext, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

/// This struct defines a clause in a match or a closure in AxCut. It consists of a name of the
/// corresponding xtor, the context it binds, and the body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub xtor: Name,
    pub context: TypingContext,
    pub body: Rc<Statement>,
}

impl FreeVars for Clause {
    fn free_vars(mut self, vars: &mut HashSet<Var>) -> Self {
        self.body = self.body.free_vars(vars);
        for binding in &self.context.bindings {
            vars.remove(&binding.var);
        }
        self
    }
}

impl Subst for Clause {
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
        let context = if self.context.bindings.is_empty() {
            alloc.nil()
        } else {
            self.context.print(cfg, alloc).parens()
        };

        alloc
            .ctor(&self.xtor)
            .append(context.group())
            .append(alloc.space())
            .append(FAT_ARROW)
            .align()
            .append(alloc.line())
            .append(self.body.print(cfg, alloc).group())
            .nest(cfg.indent)
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
                .append(
                    alloc.intersperse(
                        clauses
                            .iter()
                            .map(|clause| clause.print(cfg, alloc).group()),
                        sep,
                    ),
                )
                .nest(cfg.indent)
                .append(alloc.hardline())
                .braces_anno()
        }
    }
}
