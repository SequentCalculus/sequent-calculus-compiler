use super::{check_args, Check};
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Destructor, types::Ty},
    typing::{
        check::{check_equality, lookup_ty_for_dtor},
        errors::Error,
        symbol_table::SymbolTable,
    },
};

impl Check for Destructor {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Destructor, Error> {
        let ty = lookup_ty_for_dtor(&self.span.to_miette(), &self.id, symbol_table)?;
        let new_destructee = self.destructee.check(symbol_table, context, &ty)?;
        match symbol_table.dtors.get(&self.id) {
            Some((types, ret_ty)) => {
                let new_args = check_args(
                    &self.span.to_miette(),
                    symbol_table,
                    context,
                    self.args,
                    types,
                )?;
                check_equality(&self.span.to_miette(), expected, ret_ty)?;
                Ok(Destructor {
                    span: self.span,
                    id: self.id,
                    args: new_args,
                    destructee: new_destructee,
                })
            }
            None => Err(Error::Undefined {
                span: self.span.to_miette(),
                name: self.id.clone(),
            }),
        }
    }
}
