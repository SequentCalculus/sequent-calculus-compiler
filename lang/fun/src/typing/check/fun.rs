use super::{check_args, check_equality, terms::Check};
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Fun, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Fun {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        match symbol_table.funs.get(&self.name) {
            Some((types, ret_ty)) => {
                check_equality(&self.span.to_miette(), expected, ret_ty)?;
                let new_args = check_args(
                    &self.span.to_miette(),
                    symbol_table,
                    context,
                    self.args,
                    types,
                )?;
                Ok(Fun {
                    span: self.span,
                    name: self.name,
                    args: new_args,
                    ret_ty: Some(expected.clone()),
                })
            }
            None => Err(Error::Undefined {
                span: self.span.to_miette(),
                name: self.name.clone(),
            }),
        }
    }
}
