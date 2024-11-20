use printer::{
    theme::ThemeExt,
    tokens::{COMMA, IFE, SEMI},
    DocAllocator, Print,
};

use crate::syntax_var::{Statement, Var};
use crate::traits::{substitution::SubstVar, used_binders::UsedBinders};

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
        alloc.keyword(IFE).append(
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

impl From<IfE> for Statement {
    fn from(value: IfE) -> Self {
        Statement::IfE(value)
    }
}

impl UsedBinders for IfE {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.thenc.used_binders(used);
        self.elsec.used_binders(used);
    }
}

impl SubstVar for IfE {
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
