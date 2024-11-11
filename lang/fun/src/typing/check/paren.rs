use super::terms::Check;
use crate::{
    syntax::{context::TypingContext, terms::Paren, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Paren {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        self.inner.check(symbol_table, context, expected)
    }
}
