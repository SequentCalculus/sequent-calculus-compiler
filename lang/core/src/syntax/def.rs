use printer::{
    tokens::{COLONEQ, DEF, SEMI},
    DocAllocator, Print,
};

use super::{context::TypingContext, Name, Statement};

#[derive(Debug, Clone, PartialEq)]
pub struct Def {
    pub name: Name,
    pub context: TypingContext,
    pub body: Statement,
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
