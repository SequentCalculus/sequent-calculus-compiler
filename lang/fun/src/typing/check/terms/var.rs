use super::Check;
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Var, types::Ty},
    typing::{
        check::{check_equality, lookup_var},
        errors::Error,
        symbol_table::SymbolTable,
    },
};

impl Check for Var {
    fn check(
        self,
        _symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Var, Error> {
        let found_ty = lookup_var(&self.span.to_miette(), context, &self.var)?;
        check_equality(&self.span.to_miette(), expected, &found_ty)?;
        Ok(Var {
            span: self.span,
            var: self.var,
            ty: Some(expected.clone()),
        })
    }
}
