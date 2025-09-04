//! This module defines arguments in AxCut.

use printer::{DocAllocator, Print};

use super::Var;

/// This struct defines arguments in AxCut. They consist of a list of [`Var`]s.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Arguments {
    pub entries: Vec<Var>,
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

impl From<Vec<Var>> for Arguments {
    fn from(bindings: Vec<Var>) -> Arguments {
        Arguments { entries: bindings }
    }
}
