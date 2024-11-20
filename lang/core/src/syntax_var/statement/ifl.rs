use printer::{
    theme::ThemeExt,
    tokens::{COMMA, IFL, SEMI},
    DocAllocator, Print,
};

use crate::syntax_var::{Statement, Var};
use crate::traits::{substitution::SubstVar, used_binders::UsedBinders};

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
        alloc.keyword(IFL).append(
            alloc
                .text(&self.fst)
                .append(COMMA)
                .append(alloc.space())
                .append(alloc.text(&self.snd))
                .append(SEMI)
                .append(alloc.space())
                .append(self.thenc.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(self.elsec.print(cfg, alloc))
                .parens(),
        )
    }
}

impl From<IfL> for Statement {
    fn from(value: IfL) -> Self {
        Statement::IfL(value)
    }
}

impl UsedBinders for IfL {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.thenc.used_binders(used);
        self.elsec.used_binders(used);
    }
}

impl SubstVar for IfL {
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
