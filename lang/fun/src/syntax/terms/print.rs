use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{PRINTLN_I64, SEMI},
    DocAllocator, Print,
};

use super::Term;
use crate::{
    syntax::{
        context::TypingContext,
        types::{OptTyped, Ty},
        Variable,
    },
    traits::UsedBinders,
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use std::{collections::HashSet, rc::Rc};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct PrintLnI64 {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub term: Rc<Term>,
    pub case: Rc<Term>,
    pub ty: Option<Ty>,
}

impl OptTyped for PrintLnI64 {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for PrintLnI64 {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(PRINTLN_I64)
            .append(self.term.print(cfg, alloc).parens())
            .append(alloc.keyword(SEMI))
            .append(alloc.line())
            .append(self.case.print(cfg, alloc))
            .align()
    }
}

impl From<PrintLnI64> for Term {
    fn from(value: PrintLnI64) -> Self {
        Term::PrintLnI64(value)
    }
}

impl Check for PrintLnI64 {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let term_checked = self.term.check(symbol_table, context, &Ty::mk_i64())?;
        let case_checked = self.case.check(symbol_table, context, expected)?;
        Ok(PrintLnI64 {
            term: term_checked,
            case: case_checked,
            ty: Some(expected.clone()),
            ..self
        })
    }
}

impl UsedBinders for PrintLnI64 {
    fn used_binders(&self, used: &mut HashSet<Variable>) {
        self.term.used_binders(used);
        self.case.used_binders(used);
    }
}
