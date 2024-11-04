use super::Check;
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Lit, types::Ty},
    typing::{check::check_equality, errors::Error, symbol_table::SymbolTable},
};

impl Check for Lit {
    fn check(
        self,
        _symbol_table: &SymbolTable,
        _context: &TypingContext,
        expected: &Ty,
    ) -> Result<Lit, Error> {
        check_equality(&self.span.to_miette(), expected, &Ty::mk_int())?;
        Ok(Lit {
            span: self.span,
            val: self.val,
        })
    }
}
