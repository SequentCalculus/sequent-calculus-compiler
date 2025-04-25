use codespan::Span;
use derivative::Derivative;
use printer::{DocAllocator, Print, theme::ThemeExt, tokens::EXIT};

use super::Term;
use crate::{
    syntax::{
        Var,
        context::TypingContext,
        types::{OptTyped, Ty},
    },
    traits::used_binders::UsedBinders,
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use std::{collections::HashSet, rc::Rc};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Exit {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub arg: Rc<Term>,
    pub ty: Option<Ty>,
}

impl Print for Exit {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(EXIT)
            .append(alloc.space())
            .append(self.arg.print(cfg, alloc))
    }
}

impl OptTyped for Exit {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl From<Exit> for Term {
    fn from(value: Exit) -> Self {
        Term::Exit(value)
    }
}

impl Check for Exit {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        self.arg = self.arg.check(symbol_table, context, &Ty::mk_i64())?;

        self.ty = Some(expected.clone());
        Ok(self)
    }
}

impl UsedBinders for Exit {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.arg.used_binders(used);
    }
}
