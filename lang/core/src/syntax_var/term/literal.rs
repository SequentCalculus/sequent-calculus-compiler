use printer::{DocAllocator, Print};

use super::Term;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub lit: i64,
}

impl Literal {
    #[must_use]
    pub fn new(lit: i64) -> Self {
        Literal { lit }
    }
}

impl Print for Literal {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(format!("{}", self.lit))
    }
}

impl From<Literal> for Term {
    fn from(value: Literal) -> Self {
        Term::Literal(value)
    }
}
