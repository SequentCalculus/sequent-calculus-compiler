use printer::{tokens::FAT_ARROW, DocAllocator, Print};

use super::{FsStatement, FsTypingContext, Name, Var};
use crate::traits::substitution::SubstVar;

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsClause {
    pub xtor: Name,
    pub context: FsTypingContext,
    pub case: Rc<FsStatement>,
}

impl Print for FsClause {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let params = if self.context.is_empty() {
            alloc.nil()
        } else {
            self.context.print(cfg, alloc).parens()
        };
        let prefix = alloc
            .text(&self.xtor)
            .append(params)
            .append(alloc.space())
            .append(FAT_ARROW);
        let tail = alloc
            .line()
            .append(self.case.print(cfg, alloc))
            .nest(cfg.indent);
        prefix.append(tail).group()
    }
}

impl SubstVar for FsClause {
    type Target = FsClause;

    fn subst_sim(self, subst: &[(Var, Var)]) -> FsClause {
        FsClause {
            case: self.case.subst_sim(subst),
            ..self
        }
    }
}
