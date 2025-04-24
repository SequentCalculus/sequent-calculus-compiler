use codespan::Span;
use derivative::Derivative;
use printer::{
    Alloc, Builder, DocAllocator, Print, PrintCfg,
    theme::ThemeExt,
    tokens::{COMMA, FAT_ARROW},
    util::BracesExt,
};

use super::Term;
use crate::{
    syntax::{
        Name, Var,
        context::{NameContext, TypingContext},
        declarations::Polarity,
        types::{OptTyped, Ty},
    },
    traits::used_binders::UsedBinders,
};

use std::collections::HashSet;

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Clause {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// Whether we have a clause of a case or new expression.
    pub pol: Polarity,
    pub xtor: Name,
    pub context_names: NameContext,
    pub context: TypingContext,
    pub body: Term,
}

impl OptTyped for Clause {
    fn get_type(&self) -> Option<Ty> {
        self.body.get_type()
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
            .append(self.body.print(cfg, alloc))
    }
}

pub fn print_clauses<'a>(
    clauses: &'a [Clause],
    cfg: &PrintCfg,
    alloc: &'a Alloc<'a>,
) -> Builder<'a> {
    match clauses.len() {
        0 => alloc.space().braces_anno(),
        1 => alloc
            .line()
            .append(clauses[0].print(cfg, alloc))
            .nest(cfg.indent)
            .append(alloc.line())
            .braces_anno()
            .group(),
        _ => {
            let sep = alloc.text(COMMA).append(alloc.hardline());
            alloc
                .hardline()
                .append(alloc.intersperse(clauses.iter().map(|x| x.print(cfg, alloc)), sep.clone()))
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
        self.body.used_binders(used);
    }
}
