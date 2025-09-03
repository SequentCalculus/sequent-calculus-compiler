//! This module defines printing an integer in Fun.

use codespan::Span;
use derivative::Derivative;
use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{PRINT_I64, PRINTLN_I64, SEMI},
};

use super::Term;
use crate::{
    syntax::{
        context::TypingContext,
        names::Var,
        types::{OptTyped, Ty},
    },
    traits::used_binders::UsedBinders,
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use std::{collections::HashSet, rc::Rc};

/// This struct defines printing an integer in Fun. It consists of the information whether a
/// newline should be printed, the term for the integer to print, the remaining statement, and
/// after typechecking also of the inferred type.
///
/// Example:
/// `println_i64(x); 1` prints the integer bound to `x` and a newline and then returns `1`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct PrintI64 {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// Whether to print a newline after the value
    pub newline: bool,
    /// The term for the integer to be printed
    pub arg: Rc<Term>,
    /// The next term after the print
    pub next: Rc<Term>,
    /// The (inferred) type of the term
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
            .append(
                alloc
                    .line_()
                    .append(self.arg.print(cfg, alloc).group())
                    .nest(cfg.indent)
                    .append(alloc.line_())
                    .parens()
                    .group(),
            )
            .append(SEMI)
            .append(alloc.hardline())
            .append(self.next.print(cfg, alloc).group())
    }
}

impl From<PrintI64> for Term {
    fn from(value: PrintI64) -> Self {
        Term::PrintI64(value)
    }
}

impl Check for PrintI64 {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        self.arg = self.arg.check(symbol_table, context, &Ty::mk_i64())?;

        self.next = self.next.check(symbol_table, context, expected)?;

        self.ty = Some(expected.clone());
        Ok(self)
    }
}

impl UsedBinders for PrintI64 {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.arg.used_binders(used);
        self.next.used_binders(used);
    }
}
