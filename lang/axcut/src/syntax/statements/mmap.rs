use printer::theme::ThemeExt;
use printer::tokens::{LEFT_ARROW, MMAP_ANONYMOUS_PAGE, MUNMAP_PAGE, SEMI};
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{names::filter_by_set, Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MMapAnonymousPage {
    pub var: Var,
    pub case: Rc<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MUnmapPage {
    pub var: Var,
    pub case: Rc<Statement>,
}

impl Print for MMapAnonymousPage {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(&self.var)
            .append(alloc.space())
            .append(LEFT_ARROW)
            .append(alloc.space())
            .append(alloc.keyword(MMAP_ANONYMOUS_PAGE))
            .append(SEMI)
            .append(alloc.line())
            .append(self.case.print(cfg, alloc))
    }
}

impl Print for MUnmapPage {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(MUNMAP_PAGE)
            .append(alloc.space())
            .append(&self.var)
            .append(SEMI)
            .append(alloc.line())
            .append(self.case.print(cfg, alloc))
    }
}

impl From<MMapAnonymousPage> for Statement {
    fn from(value: MMapAnonymousPage) -> Self {
        Statement::MMapAnonymousPage(value)
    }
}

impl From<MUnmapPage> for Statement {
    fn from(value: MUnmapPage) -> Self {
        Statement::MUnmapPage(value)
    }
}

impl FreeVars for MMapAnonymousPage {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.case.free_vars(vars);
        vars.remove(&self.var);
    }
}

impl FreeVars for MUnmapPage {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.case.free_vars(vars);
        vars.insert(self.var.clone());
    }
}

impl Subst for MMapAnonymousPage {
    type Target = MMapAnonymousPage;

    fn subst_sim(self, subst: &[(Var, Var)]) -> MMapAnonymousPage {
        MMapAnonymousPage {
            case: self.case.subst_sim(subst),
            ..self
        }
    }
}

impl Subst for MUnmapPage {
    type Target = MUnmapPage;

    fn subst_sim(self, subst: &[(Var, Var)]) -> MUnmapPage {
        MUnmapPage {
            var: self.var.subst_sim(subst),
            case: self.case.subst_sim(subst),
        }
    }
}

impl Linearizing for MMapAnonymousPage {
    type Target = Statement;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars = HashSet::new();
        self.case.free_vars(&mut free_vars);

        let mut new_context = filter_by_set(&context, &free_vars);
        let context_rearrange = new_context.clone();

        new_context.push(self.var.clone());
        let mmap_anonymous_page = MMapAnonymousPage {
            var: self.var,
            case: self.case.linearize(new_context, used_vars),
        }
        .into();

        if context == context_rearrange {
            mmap_anonymous_page
        } else {
            let rearrange = context_rearrange
                .clone()
                .into_iter()
                .zip(context_rearrange)
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(mmap_anonymous_page),
            }
            .into()
        }
    }
}

impl Linearizing for MUnmapPage {
    type Target = MUnmapPage;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> MUnmapPage {
        MUnmapPage {
            var: self.var,
            case: self.case.linearize(context.clone(), used_vars),
        }
    }
}
