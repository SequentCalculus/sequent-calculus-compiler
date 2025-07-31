//! This module defines top-level functions in Core.

use std::collections::HashSet;

use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{COLONEQ, DEF, SEMI},
};

use super::{FsStatement, Name, Statement, Var, context::TypingContext};
use crate::traits::*;

/// This struct defines top-level function definitions. A top-level function consists of a name
/// (unique in the program), a typing context defining the parameters, and the body statement. It
/// is annotated with the list of all variable names used in the top-level function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Def {
    /// The name of the definition
    pub name: Name,
    /// The parameter context
    pub context: TypingContext,
    /// The body statement
    pub body: Statement,
    /// Variable names used in the top-level function
    pub used_vars: HashSet<Var>,
}

impl Def {
    /// This function applies the [`Focusing`] transformation to the body of the top-level function.
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

/// This struct defines the focused version of top-level function [`Def`]efinitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsDef {
    /// The name of the definition
    pub name: Name,
    /// The parameter context
    pub context: TypingContext,
    /// The body statement
    pub body: FsStatement,
    /// Variable names used in the top-level function
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
