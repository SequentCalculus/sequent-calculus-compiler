use printer::{tokens::FAT_ARROW, DocAllocator, Print};

use super::{Name, Statement, TypingContext, Var};
use crate::traits::{substitution::SubstVar, used_binders::UsedBinders};

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub xtor: Name,
    pub context: TypingContext,
    pub case: Rc<Statement>,
}

impl Print for Clause {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(&self.xtor)
            .append(self.context.print(cfg, alloc).parens())
            .append(alloc.space())
            .append(FAT_ARROW)
            .append(alloc.space())
            .append(self.case.print(cfg, alloc))
    }
}

impl UsedBinders for Clause {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        for binding in &self.context {
            used.insert(binding.var.clone());
        }
        self.case.used_binders(used);
    }
}

impl SubstVar for Clause {
    type Target = Clause;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Clause {
        Clause {
            case: self.case.subst_sim(subst),
            ..self
        }
    }
}
