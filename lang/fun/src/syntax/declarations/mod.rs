//! This module contains top-level declarations of [`Data`] and [`Codata`] types and of top-level
//! function [`Def`]initions.

use printer::*;

pub mod codata;
pub mod data;
pub mod def;
pub mod exports;
pub mod import;

pub use codata::*;
pub use data::*;
pub use def::*;
pub use exports::*;
pub use import::*;

/// This enum encodes whether a user-declared type is a data or codata type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Polarity {
    Data,
    Codata,
}

// TODO: contemplate boxing large variants here
#[allow(clippy::large_enum_variant)]
/// This enum defines top-level declarations. They are either [`Data`] or [`Codata`] type templates
/// or top-level function [`Def`]initions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    Exports(Exports),
    Import(Import),
    Data(Data),
    Codata(Codata),
    Def(Def),
}

impl Print for Declaration {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        match self {
            Declaration::Exports(exports) => exports.print(cfg, alloc),
            Declaration::Import(import) => import.print(cfg, alloc),
            Declaration::Def(def) => def.print(cfg, alloc),
            Declaration::Data(data) => data.print(cfg, alloc),
            Declaration::Codata(codata) => codata.print(cfg, alloc),
        }
    }
}
