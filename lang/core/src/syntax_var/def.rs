use printer::{
    theme::ThemeExt,
    tokens::{COLONEQ, DEF, SEMI},
    DocAllocator, Print,
};

use crate::syntax::{statement::FsStatement, Name, Var};

use super::FsTypingContext;

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct FsDef {
    pub name: Name,
    pub context: FsTypingContext,
    pub body: FsStatement,
    pub used_vars: HashSet<Var>,
}

impl Print for FsDef {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let params = if self.context.is_empty() {
            alloc.nil()
        } else {
            self.context.print(cfg, alloc).parens()
        };
        let head = alloc
            .keyword(DEF)
            .append(alloc.space())
            .append(alloc.text(&self.name))
            .append(params)
            .append(alloc.space())
            .append(COLONEQ);
        let body = alloc
            .line()
            .append(self.body.print(cfg, alloc))
            .append(SEMI)
            .nest(cfg.indent);
        head.append(body).group()
    }
}
