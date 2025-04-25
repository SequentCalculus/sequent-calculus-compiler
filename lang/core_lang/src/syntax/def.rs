use std::collections::HashSet;

use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{COLONEQ, DEF, SEMI},
};

use super::{FsStatement, Name, Statement, Var, context::TypingContext};
use crate::traits::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Def {
    pub name: Name,
    pub context: TypingContext,
    pub body: Statement,
    pub used_vars: HashSet<Var>,
}

impl Def {
    pub fn focus(mut self) -> FsDef {
        FsDef {
            name: self.name,
            context: self.context,
            body: self.body.focus(&mut self.used_vars),
            used_vars: self.used_vars,
        }
    }
}

impl Print for Def {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let params = if self.context.bindings.is_empty() {
            alloc.nil()
        } else {
            self.context.bindings.print(cfg, alloc).parens()
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsDef {
    pub name: Name,
    pub context: TypingContext,
    pub body: FsStatement,
    pub used_vars: HashSet<Var>,
}

impl Print for FsDef {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let params = if self.context.bindings.is_empty() {
            alloc.nil()
        } else {
            self.context.bindings.print(cfg, alloc).parens()
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
