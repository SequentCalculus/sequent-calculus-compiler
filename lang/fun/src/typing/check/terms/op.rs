use super::Check;
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Op, types::Ty},
    typing::{check::check_equality, errors::Error, symbol_table::SymbolTable},
};

impl Check for Op {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Op, Error> {
        check_equality(&self.span.to_miette(), &Ty::mk_int(), expected)?;
        // In the following two cases we know that "expected = Int".
        let fst_checked = self.fst.check(symbol_table, context, expected)?;
        let snd_checked = self.snd.check(symbol_table, context, expected)?;
        Ok(Op {
            span: self.span,
            fst: fst_checked,
            op: self.op,
            snd: snd_checked,
        })
    }
}
