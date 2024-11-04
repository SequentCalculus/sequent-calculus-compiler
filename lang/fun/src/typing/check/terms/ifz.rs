use super::Check;
use crate::{
    syntax::{context::TypingContext, terms::IfZ, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for IfZ {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<IfZ, Error> {
        let ifc = self.ifc.check(symbol_table, context, &Ty::mk_int())?;
        let thenc = self.thenc.check(symbol_table, context, expected)?;
        let elsec = self.elsec.check(symbol_table, context, expected)?;
        Ok(IfZ {
            span: self.span,
            ifc,
            thenc,
            elsec,
        })
    }
}
