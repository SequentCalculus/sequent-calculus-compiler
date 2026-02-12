//! This module defines a clause in a [match](crate::syntax::terms::Case) or a
//! [comatch](crate::syntax::terms::New) in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{COMMA, FAT_ARROW};
use printer::*;

use crate::syntax::*;
use crate::traits::*;

use std::collections::HashSet;

/// This struct defines a clause in a match or a comatch in Fun. It consists of a
/// [polarity](Polarity) that determines whether it is in a match (of a data type) or a comatch
/// (of a codata type), of a name of the corresponding xtor, of the context it binds for the
/// arguments, and of the body.
///
/// Example:
/// ```text
/// Cons(x, xs) => 1 + len(xs)
/// ```
/// Matches the constructor `Cons` with arguments `x` and `xs` that can be used in the body.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Clause {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// Whether we have a clause of a match or comatch
    pub pol: Polarity,
    /// The name of the xtor
    pub xtor: Name,
    /// The names (without types) to which the arguments of the xtor are bound
    pub context_names: NameContext,
    /// The bindings (with types) to which the arguments of the xtor are bound
    pub context: TypingContext,
    /// The body of the pattern
    pub body: Term,
}

impl OptTyped for Clause {
    fn get_type(&self) -> Option<Ty> {
        self.body.get_type()
    }
}

impl Print for Clause {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let xtor = match self.pol {
            Polarity::Data => alloc.ctor(&self.xtor),
            Polarity::Codata => alloc.dtor(&self.xtor),
        };
        xtor.append(self.context_names.print(cfg, alloc))
            .append(alloc.space())
            .append(FAT_ARROW)
            .align()
            .append(alloc.line())
            .append(self.body.print(cfg, alloc).group())
            .nest(cfg.indent)
    }
}

pub fn print_clauses<'a>(
    clauses: &'a [Clause],
    cfg: &PrintCfg,
    alloc: &'a Alloc<'a>,
) -> Builder<'a> {
    match clauses.len() {
        0 => alloc.space().braces_anno().group(),
        1 => alloc
            .line()
            .append(clauses[0].print(cfg, alloc).group())
            .nest(cfg.indent)
            .append(alloc.line())
            .braces_anno()
            .group(),
        _ => {
            let sep = alloc.text(COMMA).append(alloc.hardline());
            alloc
                .hardline()
                .append(
                    alloc.intersperse(
                        clauses
                            .iter()
                            .map(|clause| clause.print(cfg, alloc).group()),
                        sep,
                    ),
                )
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
