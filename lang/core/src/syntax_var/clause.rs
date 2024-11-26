use printer::tokens::COMMA;
use printer::util::BracesExt;
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

pub fn print_clauses<'a>(
    cases: &'a [FsClause],
    cfg: &printer::PrintCfg,
    alloc: &'a printer::Alloc<'a>,
) -> printer::Builder<'a> {
    match cases.len() {
        0 => alloc.space().braces_anno(),

        1 => alloc
            .line()
            .append(cases[0].print(cfg, alloc))
            .nest(cfg.indent)
            .append(alloc.line())
            .braces_anno()
            .group(),
        _ => {
            let sep = alloc.text(COMMA).append(alloc.hardline());
            alloc
                .hardline()
                .append(alloc.intersperse(cases.iter().map(|x| x.print(cfg, alloc)), sep.clone()))
                .nest(cfg.indent)
                .append(alloc.hardline())
                .braces_anno()
        }
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
