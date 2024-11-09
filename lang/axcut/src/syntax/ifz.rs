use printer::theme::ThemeExt;
use printer::tokens::{COMMA, FAT_ARROW, IFZ};
use printer::util::BracesExt;
use printer::{DocAllocator, Print};

use super::{Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use std::collections::HashSet;

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfZ {
    pub ifc: Var,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl Print for IfZ {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(IFZ)
            .append(alloc.space())
            .append(self.ifc.print(cfg, alloc))
            .append(alloc.space())
            .append(
                alloc
                    .text("()")
                    .append(alloc.space())
                    .append(alloc.text(FAT_ARROW))
                    .append(alloc.space())
                    .append(self.thenc.print(cfg, alloc))
                    .append(COMMA)
                    .append("()")
                    .append(alloc.space())
                    .append(alloc.text(FAT_ARROW))
                    .append(alloc.space())
                    .append(self.elsec.print(cfg, alloc))
                    .braces_anno(),
            )
    }
}

impl From<IfZ> for Statement {
    fn from(value: IfZ) -> Self {
        Statement::IfZ(value)
    }
}

impl FreeVars for IfZ {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.thenc.free_vars(vars);
        self.elsec.free_vars(vars);
        vars.insert(self.ifc.clone());
    }
}

impl Subst for IfZ {
    type Target = IfZ;

    fn subst_sim(self, subst: &[(Var, Var)]) -> IfZ {
        IfZ {
            ifc: self.ifc.subst_sim(subst),
            thenc: self.thenc.subst_sim(subst),
            elsec: self.elsec.subst_sim(subst),
        }
    }
}

impl UsedBinders for IfZ {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.thenc.used_binders(used);
        self.elsec.used_binders(used);
    }
}

impl Linearizing for IfZ {
    type Target = crate::syntax::IfZ;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> crate::syntax::IfZ {
        crate::syntax::IfZ {
            ifc: self.ifc,
            thenc: self.thenc.linearize(context.clone(), used_vars),
            elsec: self.elsec.linearize(context, used_vars),
        }
    }
}
