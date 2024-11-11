use super::{check_args, check_equality, declarations::lookup_ty_for_dtor, terms::Check};
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Destructor, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Destructor {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Destructor, Error> {
        let ty = lookup_ty_for_dtor(&self.span.to_miette(), &self.id, symbol_table)?;
        let destructee_checked = self.destructee.check(symbol_table, context, &ty)?;
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
                    destructee: destructee_checked,
                    args: new_args,
                    ty: Some(expected.clone()),
                })
            }
            None => Err(Error::Undefined {
                span: self.span.to_miette(),
                name: self.id.clone(),
            }),
        }
    }
}
