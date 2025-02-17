use super::{Name, Statement, TypingContext, Var};
use printer::{
    theme::ThemeExt,
    tokens::{COLONEQ, DEF},
    DocAllocator, Print,
};

use crate::traits::linearize::Linearizing;

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Def {
    pub name: Name,
    pub context: TypingContext,
    pub body: Statement,
    pub used_vars: HashSet<Var>,
}

impl Def {
    pub fn linearize(mut self) -> Def {
        self.body = self
            .body
            .linearize(self.context.vars(), &mut self.used_vars);
        self
    }
}

impl Print for Def {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let head = alloc
            .keyword(DEF)
            .append(alloc.space())
            .append(&self.name)
            .append(self.context.print(cfg, alloc))
            .append(alloc.space())
            .append(COLONEQ);
        let body = alloc
            .line()
            .append(self.body.print(cfg, alloc))
            .nest(cfg.indent);
        head.append(body).group()
    }
}
