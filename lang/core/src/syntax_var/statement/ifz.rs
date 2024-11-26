use printer::{
    theme::ThemeExt,
    tokens::{COMMA, IFZ, SEMI},
    DocAllocator, Print,
};

use crate::syntax_var::{FsStatement, Var};
use crate::traits::substitution::SubstVar;

use std::rc::Rc;

/// Focused IfZ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsIfZ {
    pub ifc: Var,
    pub thenc: Rc<FsStatement>,
    pub elsec: Rc<FsStatement>,
}

impl Print for FsIfZ {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword(IFZ).append(
            alloc
                .text(&self.ifc)
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

impl From<FsIfZ> for FsStatement {
    fn from(value: FsIfZ) -> Self {
        FsStatement::IfZ(value)
    }
}

impl SubstVar for FsIfZ {
    type Target = FsIfZ;

    fn subst_sim(self, subst: &[(Var, Var)]) -> FsIfZ {
        FsIfZ {
            ifc: self.ifc.subst_sim(subst),
            thenc: self.thenc.subst_sim(subst),
            elsec: self.elsec.subst_sim(subst),
        }
    }
}
