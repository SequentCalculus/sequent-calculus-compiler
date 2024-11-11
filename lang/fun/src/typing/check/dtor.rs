use super::{check_args, check_equality, lookup_ty_for_dtor, terms::Check};
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Destructor, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Destructor {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        let ty = lookup_ty_for_dtor(&self.span.to_miette(), &self.id, symbol_table)?;
        self.destructee.check(symbol_table, context, &ty)?;
        match symbol_table.dtors.get(&self.id) {
            Some((types, ret_ty)) => {
                check_args(
                    &self.span.to_miette(),
                    symbol_table,
                    context,
                    &self.args,
                    types,
                )?;
                check_equality(&self.span.to_miette(), expected, ret_ty)
            }
            None => Err(Error::Undefined {
                span: self.span.to_miette(),
                name: self.id.clone(),
            }),
        }
    }
}
