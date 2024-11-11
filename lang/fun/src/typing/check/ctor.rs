use super::{check_args, check_equality, declarations::lookup_ty_for_ctor, terms::Check};
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Constructor, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Constructor {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Constructor, Error> {
        match symbol_table.ctors.get(&self.id) {
            Some(types) => {
                let new_args = check_args(
                    &self.span.to_miette(),
                    symbol_table,
                    context,
                    self.args,
                    types,
                )?;
                let (ty, _) = lookup_ty_for_ctor(&self.span.to_miette(), &self.id, symbol_table)?;
                check_equality(&self.span.to_miette(), expected, &ty)?;
                Ok(Constructor {
                    span: self.span,
                    id: self.id,
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
