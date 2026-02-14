//! This module defines top-level functions in Core.

use printer::tokens::DEF;
use printer::*;

use crate::syntax::*;
use crate::traits::*;

use std::collections::HashSet;

/// This struct defines top-level function definitions. A top-level function consists of a name
/// (unique in the program), a typing context defining the parameters, and the body statement. It
/// is annotated with the list of all variable names used in the top-level function. The type
/// parameter `S` determines whether this is the unfocused variant (if `S` is instantiated with
/// [`Statement`], which is the default) or the focused variant (if `S` is instantiated with
/// [`FsStatement`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Def<S = Statement> {
    /// The name of the definition
    pub name: Ident,
    /// The parameter context
    pub context: TypingContext,
    /// The body statement
    pub body: S,
    /// Variable names used in the top-level function
    pub used_vars: HashSet<Ident>,
}

pub type FsDef = Def<FsStatement>;

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

impl<S: Print> Print for Def<S> {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let head = alloc
            .keyword(DEF)
            .append(alloc.space())
            .append(self.name.print(cfg, alloc))
            .append(self.context.print(cfg, alloc).parens())
            .append(alloc.space());

        let body = alloc
            .hardline()
            .append(self.body.print(cfg, alloc).group())
            .nest(cfg.indent)
            .append(alloc.hardline())
            .braces_anno();

        head.group().append(body)
    }
}
