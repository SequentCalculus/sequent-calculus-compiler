use super::{lookup_covar, terms::Check};
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Goto, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Goto {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        _expected: &Ty,
    ) -> Result<(), Error> {
        let cont_type = lookup_covar(&self.span.to_miette(), context, &self.target)?;
        self.term.check(symbol_table, context, &cont_type)
    }
}
