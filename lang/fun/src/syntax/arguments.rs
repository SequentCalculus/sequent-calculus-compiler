//! This module defines arguments in Fun.

use printer::{DocAllocator, Print};

use super::terms::Term;

/// This struct defines arguments in Fun. They consist of a list of [`Term`]s.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Arguments {
    pub entries: Vec<Term>,
}

impl Print for Arguments {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let sep = if cfg.allow_linebreaks {
            alloc.line_()
        } else {
            alloc.nil()
        };

        if self.entries.is_empty() {
            alloc.nil()
        } else {
            sep.clone()
                .append(self.entries.print(cfg, alloc))
                .nest(cfg.indent)
                .append(sep)
        }
    }
}

impl From<Vec<Term>> for Arguments {
    fn from(bindings: Vec<Term>) -> Arguments {
        Arguments { entries: bindings }
    }
}
