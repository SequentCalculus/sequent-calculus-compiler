use super::terms::Check;
use crate::{
    syntax::{
        context::{ContextBinding, TypingContext},
        terms::Let,
        types::Ty,
    },
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Let {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        self.bound_term.check(symbol_table, context, &self.var_ty)?;
        let mut new_context = context.clone();
        new_context.push(ContextBinding::TypedVar {
            var: self.variable.clone(),
            ty: self.var_ty.clone(),
        });
        self.in_term.check(symbol_table, &new_context, expected)?;
        Ok(())
    }
}
