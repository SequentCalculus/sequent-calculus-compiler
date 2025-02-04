use printer::theme::ThemeExt;
use printer::tokens::{ELSE, EQQ, IF, LT, LTE, NEQ};
use printer::util::BracesExt;
use printer::{DocAllocator, Print};

use crate::syntax::{Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfSort {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfC {
    pub sort: IfSort,
    pub fst: Var,
    pub snd: Var,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl Print for IfC {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let comparison = match self.sort {
            IfSort::Equal => EQQ,
            IfSort::NotEqual => NEQ,
            IfSort::Less => LT,
            IfSort::LessOrEqual => LTE,
        };
        alloc
            .keyword(IF)
            .append(alloc.space())
            .append(self.fst.print(cfg, alloc))
            .append(alloc.space())
            .append(comparison)
            .append(alloc.space())
            .append(self.snd.print(cfg, alloc))
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.thenc.print(cfg, alloc))
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
            )
            .append(alloc.space())
            .append(alloc.keyword(ELSE))
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.elsec.print(cfg, alloc))
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
            )
    }
}

impl From<IfC> for Statement {
    fn from(value: IfC) -> Self {
        Statement::IfC(value)
    }
}

impl FreeVars for IfC {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.thenc.free_vars(vars);
        self.elsec.free_vars(vars);
        vars.insert(self.fst.clone());
        vars.insert(self.snd.clone());
    }
}

impl Subst for IfC {
    type Target = IfC;

    fn subst_sim(self, subst: &[(Var, Var)]) -> IfC {
        IfC {
            sort: self.sort,
            fst: self.fst.subst_sim(subst),
            snd: self.snd.subst_sim(subst),
            thenc: self.thenc.subst_sim(subst),
            elsec: self.elsec.subst_sim(subst),
        }
    }
}

impl Linearizing for IfC {
    type Target = IfC;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> IfC {
        IfC {
            sort: self.sort,
            fst: self.fst,
            snd: self.snd,
            thenc: self.thenc.linearize(context.clone(), used_vars),
            elsec: self.elsec.linearize(context, used_vars),
        }
    }
}
