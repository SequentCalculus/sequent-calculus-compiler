use super::{check_args, check_equality, terms::Check};
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Fun, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Fun {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        match symbol_table.funs.get(&self.name) {
            Some((types, ret_ty)) => {
                check_equality(&self.span.to_miette(), expected, ret_ty)?;
                check_args(
                    &self.span.to_miette(),
                    symbol_table,
                    context,
                    &self.args,
                    types,
                )
            }
            None => Err(Error::Undefined {
                span: self.span.to_miette(),
                name: self.name.clone(),
            }),
        }
    }
}
