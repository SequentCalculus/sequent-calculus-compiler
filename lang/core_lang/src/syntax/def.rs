use std::collections::HashSet;

use printer::{
    theme::ThemeExt,
    tokens::{COLONEQ, DEF, SEMI},
    DocAllocator, Print,
};

use super::{context::TypingContext, statement::FsStatement, Name, Statement, Var};
use crate::traits::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Def {
    pub name: Name,
    pub context: TypingContext,
    pub body: Statement,
    pub used_vars: HashSet<Var>,
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

impl Focusing for Def {
    type Target = FsDef;
    fn focus(self, state: &mut FocusingState) -> FsDef {
        FsDef {
            name: self.name,
            context: self.context,
            body: self.body.focus(state),
            used_vars: state.used_vars.clone(),
        }
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
