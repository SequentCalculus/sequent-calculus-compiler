use super::terms::Check;
use crate::{
    syntax::{
        context::{ContextBinding, TypingContext},
        terms::Label,
        types::Ty,
    },
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Label {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let mut new_context = context.clone();
        new_context.push(ContextBinding::TypedCovar {
            covar: self.label.clone(),
            ty: expected.clone(),
        });
        let term_checked = self.term.check(symbol_table, &new_context, expected)?;
        Ok(Label {
            span: self.span,
            label: self.label,
            term: term_checked,
            ty: Some(expected.clone()),
        })
    }
}
