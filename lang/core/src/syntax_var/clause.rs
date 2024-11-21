use printer::tokens::COMMA;
use printer::util::BracesExt;
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
    cases: &'a [Clause],
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
