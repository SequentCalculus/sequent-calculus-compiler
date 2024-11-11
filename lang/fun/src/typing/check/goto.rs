use super::{context::lookup_covar, terms::Check};
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Goto, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Goto {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let cont_type = lookup_covar(&self.span.to_miette(), context, &self.target)?;
        let term_checked = self.term.check(symbol_table, context, &cont_type)?;
        Ok(Goto {
            span: self.span,
            target: self.target,
            term: term_checked,
            ty: Some(expected.clone()),
        })
    }
}
