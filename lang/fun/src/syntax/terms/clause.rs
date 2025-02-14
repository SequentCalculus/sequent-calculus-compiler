use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{COMMA, FAT_ARROW},
    util::BracesExt,
    Alloc, Builder, DocAllocator, Print, PrintCfg,
};

use super::Term;
use crate::syntax::{
    context::{NameContext, TypingContext},
    declarations::Polarity,
    types::{OptTyped, Ty},
    used_binders::UsedBinders,
    Name, Var,
};

use std::collections::HashSet;

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Clause {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// Whether we have a clause of a case or cocase expression.
    pub pol: Polarity,
    pub xtor: Name,
    pub context_names: NameContext,
    pub context: TypingContext,
    pub rhs: Term,
}

impl OptTyped for Clause {
    fn get_type(&self) -> Option<Ty> {
        self.rhs.get_type()
    }
}

impl Print for Clause {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let xtor = match self.pol {
            Polarity::Data => alloc.ctor(&self.xtor),
            Polarity::Codata => alloc.dtor(&self.xtor),
        };
        xtor.append(self.context_names.print(cfg, alloc))
            .append(alloc.space())
            .append(FAT_ARROW)
            .append(alloc.space())
            .append(self.rhs.print(cfg, alloc))
    }
}

pub fn print_clauses<'a>(cases: &'a [Clause], cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
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
        for binding in &self.context_names.bindings {
            used.insert(binding.clone());
        }
        self.rhs.used_binders(used);
    }
}
