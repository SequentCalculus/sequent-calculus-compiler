use super::{check_equality, terms::Check};
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Op, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Op {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        check_equality(&self.span.to_miette(), &Ty::mk_int(), expected)?;
        // In the following two cases we know that "expected = Int".
        self.fst.check(symbol_table, context, expected)?;
        self.snd.check(symbol_table, context, expected)
    }
}
