//! This module defines exports for modules in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::EXPORTS;
use printer::*;


use crate::syntax::*;
use crate::typing::*;

/// This struct defines the exports statement in Fun. It consists of a term for the file/folder
/// which is part of the module.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Exports {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The file/folder
    pub name: Name,
}

impl Exports {
    pub fn check(self, _symbol_table: & SymbolTable) -> Result<Exports, Error>{
        Ok(self)
    }
}

impl Print for Exports {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        alloc
            .keyword(EXPORTS)
            .append(alloc.space())
            .append(self.name.print(cfg, alloc))
    }
}

impl From<Exports> for Declaration {
    fn from(value: Exports) -> Self {
        Declaration::Exports(value)
    }
}