use printer::theme::ThemeExt;
use printer::tokens::{COMMA, FAT_ARROW, IFE};
use printer::util::BracesExt;
use printer::{DocAllocator, Print};

use crate::syntax::{Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfE {
    pub fst: Var,
    pub snd: Var,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl Print for IfE {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(IFE)
            .append(alloc.space())
            .append(
                self.fst
                    .print(cfg, alloc)
                    .append(COMMA)
                    .append(alloc.space())
                    .append(self.snd.print(cfg, alloc))
                    .parens(),
            )
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(alloc.text("()"))
                    .append(alloc.space())
                    .append(alloc.text(FAT_ARROW))
                    .append(alloc.line().nest(cfg.indent))
                    .append(self.thenc.print(cfg, alloc).nest(cfg.indent))
                    .append(COMMA)
                    .append(alloc.line())
                    .append("()")
                    .append(alloc.space())
                    .append(alloc.text(FAT_ARROW))
                    .append(alloc.line().nest(cfg.indent))
                    .append(self.elsec.print(cfg, alloc).nest(cfg.indent))
                    .nest(cfg.indent)
                    .braces_anno(),
            )
    }
}

impl From<IfE> for Statement {
    fn from(value: IfE) -> Self {
        Statement::IfE(value)
    }
}

impl FreeVars for IfE {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.thenc.free_vars(vars);
        self.elsec.free_vars(vars);
        vars.insert(self.fst.clone());
        vars.insert(self.snd.clone());
    }
}

impl Subst for IfE {
    type Target = IfE;

    fn subst_sim(self, subst: &[(Var, Var)]) -> IfE {
        IfE {
            fst: self.fst.subst_sim(subst),
            snd: self.snd.subst_sim(subst),
            thenc: self.thenc.subst_sim(subst),
            elsec: self.elsec.subst_sim(subst),
        }
    }
}

impl UsedBinders for IfE {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.thenc.used_binders(used);
        self.elsec.used_binders(used);
    }
}

impl Linearizing for IfE {
    type Target = IfE;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> IfE {
        IfE {
            fst: self.fst,
            snd: self.snd,
            thenc: self.thenc.linearize(context.clone(), used_vars),
            elsec: self.elsec.linearize(context, used_vars),
        }
    }
}
