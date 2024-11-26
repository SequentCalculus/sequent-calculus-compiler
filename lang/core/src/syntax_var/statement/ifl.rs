use printer::{
    theme::ThemeExt,
    tokens::{COMMA, IFL, SEMI},
    DocAllocator, Print,
};

use crate::syntax_var::{FsStatement, Var};
use crate::traits::substitution::SubstVar;

use std::rc::Rc;

/// Focused IfL
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsIfL {
    pub fst: Var,
    pub snd: Var,
    pub thenc: Rc<FsStatement>,
    pub elsec: Rc<FsStatement>,
}

impl Print for FsIfL {
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

impl From<FsIfL> for FsStatement {
    fn from(value: FsIfL) -> Self {
        FsStatement::IfL(value)
    }
}

impl SubstVar for FsIfL {
    type Target = FsIfL;

    fn subst_sim(self, subst: &[(Var, Var)]) -> FsIfL {
        FsIfL {
            fst: self.fst.subst_sim(subst),
            snd: self.snd.subst_sim(subst),
            thenc: self.thenc.subst_sim(subst),
            elsec: self.elsec.subst_sim(subst),
        }
    }
}
