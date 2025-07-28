//! Defines [Clause]
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

/// A Pattern in a [case][crate::syntax::terms::Case] or [new][crate::syntax::terms::New] expression
/// Example: `Cons(x,xs) => 1 + len(xs)`
/// Matches the constructor `Cons` with arguments `x` and `xs`
/// Example: `Tl => repeat(x)`
/// Matches the destructor `Tl`
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Clause {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// Whether we have a clause of a case or new expression.
    pub pol: Polarity,
    /// The name of the bound construcotr or destructor
    pub xtor: Name,
    /// The arguments of the constructor or destructor (without types)
    pub context_names: NameContext,
    /// The arguments of the constructor or destructor (with types)
    pub context: TypingContext,
    /// The right-hand side of the pattern
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
