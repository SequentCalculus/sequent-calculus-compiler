use super::Check;
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Goto, types::Ty},
    typing::{check::lookup_covar, errors::Error, symbol_table::SymbolTable},
};

impl Check for Goto {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        _expected: &Ty,
    ) -> Result<Goto, Error> {
        let cont_type = lookup_covar(&self.span.to_miette(), context, &self.target)?;
        let term = self.term.check(symbol_table, context, &cont_type)?;
        Ok(Goto {
            span: self.span,
            term,
            target: self.target,
        })
    }
}
