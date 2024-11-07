use codespan::Span;
use derivative::Derivative;
use printer::{DocAllocator, Print};

use crate::syntax::{types::Ty, Variable};

use super::Term;

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Var {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub var: Variable,
    pub ty: Option<Ty>,
}

impl Var {
    pub fn mk(var: &str) -> Self {
        Var {
            span: Span::default(),
            var: var.to_string(),
            ty: None,
        }
    }
}

impl Print for Var {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(self.var.clone())
    }
}

impl From<Var> for Term {
    fn from(value: Var) -> Self {
        Term::Var(value)
    }
}
