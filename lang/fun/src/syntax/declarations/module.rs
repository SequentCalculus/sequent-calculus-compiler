//! This module defines Module for modules in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::MODULE;
use printer::*;


use crate::syntax::*;
use crate::typing::*;

/// This struct defines the Module statement in Fun. It consists of a term for the file/folder
/// which is part of the module.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Module {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The file/folder
    pub name: Name,
}

impl Module {
    pub fn check(self, _symbol_table: & SymbolTable) -> Result<Module, Error>{
        Ok(self)
    }
}

impl Print for Module {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        alloc
            .keyword(MODULE)
            .append(alloc.space())
            .append(self.name.print(cfg, alloc))
    }
}

impl From<Module> for Declaration {
    fn from(value: Module) -> Self {
        Declaration::Module(value)
    }
}