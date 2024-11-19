use printer::theme::ThemeExt;
use printer::tokens::{COMMA, FAT_ARROW, IFL};
use printer::util::BracesExt;
use printer::{DocAllocator, Print};

use crate::syntax::{Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfL {
    pub fst: Var,
    pub snd: Var,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl Print for IfL {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(IFL)
            .append(alloc.space())
            .append(
                self.fst
                    .print(cfg, alloc)
                    .append(COMMA)
                    .append(self.snd.print(cfg, alloc))
                    .parens(),
            )
            .append(alloc.space())
            .append(
                alloc
                    .text("()")
                    .append(alloc.space())
                    .append(alloc.text(FAT_ARROW))
                    .append(alloc.space())
                    .append(self.thenc.print(cfg, alloc))
                    .append(COMMA)
                    .append(alloc.space())
                    .append("()")
                    .append(alloc.space())
                    .append(alloc.text(FAT_ARROW))
                    .append(alloc.space())
                    .append(self.elsec.print(cfg, alloc))
                    .braces_anno(),
            )
    }
}

impl From<IfL> for Statement {
    fn from(value: IfL) -> Self {
        Statement::IfL(value)
    }
}

impl FreeVars for IfL {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.thenc.free_vars(vars);
        self.elsec.free_vars(vars);
        vars.insert(self.fst.clone());
        vars.insert(self.snd.clone());
    }
}

impl Subst for IfL {
    type Target = IfL;

    fn subst_sim(self, subst: &[(Var, Var)]) -> IfL {
        IfL {
            fst: self.fst.subst_sim(subst),
            snd: self.snd.subst_sim(subst),
            thenc: self.thenc.subst_sim(subst),
            elsec: self.elsec.subst_sim(subst),
        }
    }
}

impl UsedBinders for IfL {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.thenc.used_binders(used);
        self.elsec.used_binders(used);
    }
}

impl Linearizing for IfL {
    type Target = IfL;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> IfL {
        IfL {
            fst: self.fst,
            snd: self.snd,
            thenc: self.thenc.linearize(context.clone(), used_vars),
            elsec: self.elsec.linearize(context, used_vars),
        }
    }
}
