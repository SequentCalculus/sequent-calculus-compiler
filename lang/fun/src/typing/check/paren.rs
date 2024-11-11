use super::terms::Check;
use crate::{
    syntax::{context::TypingContext, terms::Paren, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Paren {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Paren, Error> {
        let inner_checked = self.inner.check(symbol_table, context, expected)?;
        Ok(Paren {
            span: self.span,
            inner: inner_checked,
        })
    }
}

#[cfg(test)]
mod parens_tests {
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
