//! This module defines import for modules in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::IMPORT;
use printer::*;


use crate::syntax::*;
use crate::typing::*;

/// This struct defines the import statement in Fun. It consists of a term for the file/folder
/// which is part of the module.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Import {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The file/folder
    pub name: Name,
}

impl Import {
    pub fn check(self, _symbol_table: & SymbolTable) -> Result<Import, Error>{
        Ok(self)
    }
}

impl Print for Import {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        alloc
            .keyword(IMPORT)
            .append(alloc.space())
            .append(self.name.print(cfg, alloc))
    }
}

impl From<Import> for ModuleDeclaration {
    fn from(value: Import) -> Self {
        ModuleDeclaration::Import(value)
    }
}