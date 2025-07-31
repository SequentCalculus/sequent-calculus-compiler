//! This module contains top-level declarations of [`Data`] and [`Codata`] types and of top-level
//! function [`Def`]initions.

use printer::Print;

pub mod codata;
pub mod data;
pub mod def;
pub use codata::*;
pub use data::*;
pub use def::*;

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
    Data(Data),
    Codata(Codata),
    Def(Def),
}

impl Print for Declaration {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Declaration::Def(def) => def.print(cfg, alloc),
            Declaration::Data(data) => data.print(cfg, alloc),
            Declaration::Codata(codata) => codata.print(cfg, alloc),
        }
    }
}
