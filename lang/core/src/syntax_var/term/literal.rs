use printer::{DocAllocator, Print};

use super::FsTerm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsLiteral {
    pub lit: i64,
}

impl FsLiteral {
    #[must_use]
    pub fn new(lit: i64) -> Self {
        FsLiteral { lit }
    }
}

impl Print for FsLiteral {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(format!("{}", self.lit))
    }
}

impl From<FsLiteral> for FsTerm {
    fn from(value: FsLiteral) -> Self {
        FsTerm::Literal(value)
    }
}
