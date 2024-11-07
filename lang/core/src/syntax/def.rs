use printer::{
    tokens::{COLONEQ, DEF, SEMI},
    DocAllocator, Print,
};

use super::{context::TypingContext, Name, Statement};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Def {
    pub name: Name,
    pub context: TypingContext,
    pub body: Statement,
}

impl std::fmt::Display for Def {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args: Vec<String> = self.context.iter().map(|bnd| format!("{bnd}")).collect();
        write!(
            f,
            "def {}({}) := {};",
            self.name,
            args.join(", "),
            self.body
        )
    }
}

impl Print for Def {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(DEF)
            .append(alloc.text(&self.name))
            .append(self.context.print(cfg, alloc).parens())
            .append(COLONEQ)
            .append(self.body.print(cfg, alloc))
            .append(SEMI)
    }
}
