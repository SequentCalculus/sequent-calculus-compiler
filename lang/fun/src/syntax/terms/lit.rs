use codespan::Span;
use derivative::Derivative;
use printer::{DocAllocator, Print};

use super::Term;

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Lit {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub val: i64,
}

impl Lit {
    pub fn mk(val: i64) -> Self {
        Lit {
            span: Span::default(),
            val,
        }
    }
}

impl Print for Lit {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(format!("{}", self.val))
    }
}

impl From<Lit> for Term {
    fn from(value: Lit) -> Self {
        Term::Lit(value)
    }
}
