//! This module contains top-level declarations of [`Data`] and [`Codata`] types and of top-level
//! function [`Def`]initions.

use printer::*;

pub mod import;
pub mod module;

pub use import::*;
pub use module::*;

#[allow(clippy::large_enum_variant)]
/// This enum defines module-level declarations. They are either [`Module`] or [`Import`] type templates
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleDeclaration {
    Module(Module),
    Import(Import),
}

impl Print for ModuleDeclaration {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        match self {
            ModuleDeclaration::Module(module) => module.print(cfg, alloc),
            ModuleDeclaration::Import(import) => import.print(cfg, alloc),
        }
    }
}
