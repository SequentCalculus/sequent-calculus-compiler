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
            terms::{Lit, Paren},
            types::Ty,
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use std::rc::Rc;
    #[test]
    fn check_parens() {
        let result = Paren {
            span: Span::default(),
            inner: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
        }
        .check(&SymbolTable::default(), &vec![], &Ty::mk_int())
        .unwrap();
        let expected = Paren {
            span: Span::default(),
            inner: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
        };
        assert_eq!(result, expected)
    }
}
