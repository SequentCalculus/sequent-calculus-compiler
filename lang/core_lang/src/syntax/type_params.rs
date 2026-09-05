//! This module defines the polarity annotation carried by declaration-site type parameters
//! (`TypeDeclaration`/`XtorSig`/`Def`), mirroring the `+`/`-` annotation on the Fun side.

use printer::*;

use crate::syntax::declaration::Polarity;
use crate::syntax::names::Identifier;

/// The runtime polarity of a single declaration-site type parameter: `Data` (positive/CBV) or
/// `Codata` (negative/CBN). Distinct from the [`Polarity`] *trait* (`declaration.rs`), which
/// marks a whole declaration at compile time via the zero-sized `Data`/`Codata` structs.
/// Implements the `Polarity` trait rather than introducing an unrelated concept,
/// so generic code written against `P: Polarity` keeps working.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamPolarity {
    Data,
    Codata,
}

impl Polarity for ParamPolarity {
    fn is_data(&self) -> bool {
        matches!(self, ParamPolarity::Data)
    }
}

impl Print for ParamPolarity {
    fn print<'a>(&'a self, _cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        match self {
            ParamPolarity::Data => alloc.text("+"),
            ParamPolarity::Codata => alloc.text("-"),
        }
    }
}

impl std::fmt::Display for ParamPolarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParamPolarity::Data => write!(f, "+"),
            ParamPolarity::Codata => write!(f, "-"),
        }
    }
}

/// A single declaration-site type parameter: an [`Identifier`] paired with its mandatory
/// declared [`ParamPolarity`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeParam {
    pub id: Identifier,
    pub polarity: ParamPolarity,
}

impl TypeParam {
    /// Projects a slice of declaration-site type parameters down to their bare `Identifier`s,
    /// discarding polarity.
    pub fn ids(params: &[TypeParam]) -> Vec<Identifier> {
        params.iter().map(|p| p.id.clone()).collect()
    }
}

impl PartialEq<Identifier> for TypeParam {
    fn eq(&self, other: &Identifier) -> bool {
        self.id == *other
    }
}

impl Print for TypeParam {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.id
            .print(cfg, alloc)
            .append(self.polarity.print(cfg, alloc))
    }
}
