//! This module defines substitutions in Fun.

use printer::{DocAllocator, Print};

use super::terms::Term;

/// This struct defines substitutions in Fun. It consists of a list of [`Term`]s.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Substitution {
    /// The substitution bindings
    pub bindings: Vec<Term>,
}

impl Print for Substitution {
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

        if self.bindings.is_empty() {
            alloc.nil()
        } else {
            sep.clone()
                .append(self.bindings.print(cfg, alloc))
                .nest(cfg.indent)
                .append(sep)
        }
    }
}

impl From<Vec<Term>> for Substitution {
    fn from(bindings: Vec<Term>) -> Substitution {
        Substitution { bindings }
    }
}
