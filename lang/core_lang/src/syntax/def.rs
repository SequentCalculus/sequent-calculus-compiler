//! Top-level Definitions in the core language
use std::collections::HashSet;

use printer::{
    theme::ThemeExt,
    tokens::{COLONEQ, DEF, SEMI},
    DocAllocator, Print,
};

use super::{context::TypingContext, FsStatement, Name, Statement, Var};
use crate::traits::*;

/// A top-level definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Def {
    /// The name of the definition
    pub name: Name,
    /// The argument context
    pub context: TypingContext,
    /// The body statement of the definition
    pub body: Statement,
    /// Variables used in the body
    pub used_vars: HashSet<Var>,
}

impl Def {
    /// Focus the definition (see [Focusing])
    /// Focuses the body using the saved `used_vars`
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

/// A focused definition (see [Focusing]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsDef {
    /// The name or the definition
    pub name: Name,
    /// The argument context
    pub context: TypingContext,
    /// The body statement, after focusing
    pub body: FsStatement,
    /// The used variables in the body
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
