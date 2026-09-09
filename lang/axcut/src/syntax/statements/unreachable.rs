//! This module defines the unreachable statement in AxCut.

use printer::{Print, theme::ThemeExt};

use crate::syntax::Statement;

/// This struct defines the unreachable statement in AxCut.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unreachable;

impl Print for Unreachable {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword("unreachable")
    }
}

impl From<Unreachable> for Statement {
    fn from(value: Unreachable) -> Self {
        Statement::Unreachable(value)
    }
}
