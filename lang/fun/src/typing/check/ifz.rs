use super::terms::Check;
use crate::{
    syntax::{context::TypingContext, terms::IfZ, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for IfZ {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
        self.ifc.check(symbol_table, context, &Ty::mk_int())?;
        self.thenc.check(symbol_table, context, expected)?;
        self.elsec.check(symbol_table, context, expected)
    }
}
