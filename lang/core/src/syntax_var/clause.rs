use printer::{tokens::FAT_ARROW, DocAllocator, Print};

use super::{Name, Statement, TypingContext, Var};

use crate::traits::substitution::SubstVar;

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

impl SubstVar for Clause {
    type Target = Clause;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Clause {
        Clause {
            case: self.case.subst_sim(subst),
            ..self
        }
    }
}
