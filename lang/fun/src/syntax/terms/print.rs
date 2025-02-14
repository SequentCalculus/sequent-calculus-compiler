use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{PRINTLN_I64, PRINT_I64, SEMI},
    DocAllocator, Print,
};

use super::Term;
use crate::{
    syntax::{
        context::TypingContext,
        types::{OptTyped, Ty},
        used_binders::UsedBinders,
        Var,
    },
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use std::{collections::HashSet, rc::Rc};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct PrintI64 {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub newline: bool,
    pub arg: Rc<Term>,
    pub next: Rc<Term>,
    pub ty: Option<Ty>,
}

impl OptTyped for PrintI64 {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for PrintI64 {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let print_i64 = if self.newline { PRINTLN_I64 } else { PRINT_I64 };
        alloc
            .keyword(print_i64)
            .append(self.arg.print(cfg, alloc).parens())
            .append(SEMI)
            .append(alloc.line())
            .append(self.next.print(cfg, alloc))
            .align()
    }
}

impl From<PrintI64> for Term {
    fn from(value: PrintI64) -> Self {
        Term::PrintI64(value)
    }
}

impl Check for PrintI64 {
    fn check(
        self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let arg_checked = self.arg.check(symbol_table, context, &Ty::mk_i64())?;
        let next_checked = self.next.check(symbol_table, context, expected)?;
        Ok(PrintI64 {
            arg: arg_checked,
            next: next_checked,
            ty: Some(expected.clone()),
            ..self
        })
    }
}

impl UsedBinders for PrintI64 {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.arg.used_binders(used);
        self.next.used_binders(used);
    }
}
