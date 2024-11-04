use super::Check;
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
        let new_inner = self.inner.check(symbol_table, context, expected)?;
        Ok(Paren {
            span: self.span,
            inner: new_inner,
        })
    }
}
