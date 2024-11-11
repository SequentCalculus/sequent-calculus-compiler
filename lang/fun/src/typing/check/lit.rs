use super::{check_equality, terms::Check};
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Lit, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Lit {
    fn check(
        self,
        _symbol_table: &SymbolTable,
        _context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        check_equality(&self.span.to_miette(), expected, &Ty::mk_int())?;
        Ok(Lit {
            span: self.span,
            val: self.val,
        })
    }
}
