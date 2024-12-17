use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::Print;

use super::Term;
use crate::{
    syntax::{
        context::TypingContext,
        types::{OptTyped, Ty},
    },
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Paren {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub inner: Rc<Term>,
}

impl Paren {
    pub fn mk<T: Into<Term>>(tm: T) -> Self {
        Paren {
            span: Span::default(),
            inner: Rc::new(tm.into()),
        }
    }
}

impl OptTyped for Paren {
    fn get_type(&self) -> Option<Ty> {
        self.inner.get_type()
    }
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

impl Check for Paren {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let inner_checked = self.inner.check(symbol_table, context, expected)?;
        Ok(Paren {
            inner: inner_checked,
            ..self
        })
    }
}

#[cfg(test)]
mod test {
    use super::Check;
    use crate::{
        syntax::{
            context::TypingContext,
            terms::{Lit, Paren},
            types::Ty,
        },
        typing::symbol_table::SymbolTable,
    };

    #[test]
    fn check_parens() {
        let result = Paren::mk(Lit::mk(1))
            .check(
                &SymbolTable::default(),
                &TypingContext::default(),
                &Ty::mk_i64(),
            )
            .unwrap();
        let expected = Paren::mk(Lit::mk(1));
        assert_eq!(result, expected)
    }
}
