//! This module defines top-level functions in AxCut.

use super::{Name, Statement, TypingContext, Var};
use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{COMMA, DEF},
    util::BracesExt,
};

use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;

use std::collections::HashSet;

/// This struct defines top-level function definitions. A top-level function consists of a name
/// (unique in the program), a typing context defining the parameters, and the body statement. It
/// is annotated with the list of all variable names used in the top-level function.
#[derive(Debug, Clone)]
pub struct Def {
    pub name: Name,
    pub context: TypingContext,
    pub body: Statement,
    /// Variable names used in the top-level function.
    pub used_vars: HashSet<Var>,
}

impl Def {
    /// This function applies the linearization procedure to the body of the top-level function.
    pub fn linearize(mut self) -> Def {
        // we only call this function to annotate the free variables for all substatements which
        // helps us to avoid computing free variables repeatedly
        self.body = self.body.free_vars(&mut HashSet::new());
        // the variables in the context of the body are the parameters of the top-level function
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
        let sep = alloc.text(COMMA).append(alloc.line());
        let params = alloc
            .line_()
            .append(
                alloc.intersperse(
                    self.context
                        .bindings
                        .iter()
                        .map(|binding| binding.print(cfg, alloc)),
                    sep,
                ),
            )
            .nest(cfg.indent)
            .append(alloc.line_())
            .parens();

        let head = alloc
            .keyword(DEF)
            .append(alloc.space())
            .append(self.name.print(cfg, alloc))
            .append(params.group())
            .append(alloc.space());

        let body = alloc
            .hardline()
            .append(self.body.print(cfg, alloc).group())
            .nest(cfg.indent)
            .append(alloc.hardline())
            .braces_anno();

        head.append(body)
    }
}
