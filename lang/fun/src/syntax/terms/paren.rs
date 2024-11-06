use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::Print;

use super::Term;

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Paren {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub inner: Rc<Term>,
}

impl Print for Paren {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.inner.print(cfg, alloc).parens()
    }
}

impl From<Paren> for Term {
    fn from(value: Paren) -> Self {
        Term::Paren(value)
    }
}
