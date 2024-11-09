use super::{stringify_and_join, Name, Statement, TypingContext, Var};
use crate::traits::linearize::Linearizing;

use printer::{
    theme::ThemeExt,
    tokens::{COLONEQ, DEF},
    DocAllocator, Print,
};

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Def {
    pub name: Name,
    pub context: TypingContext,
    pub body: Statement,
}

impl std::fmt::Display for Def {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = stringify_and_join(&self.context, ", ");
        write!(f, "def {}({}) :=\n  {}", self.name, args, self.body)
    }
}

impl Linearizing for Def {
    type Target = crate::syntax::Def;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> crate::syntax::Def {
        crate::syntax::Def {
            name: self.name,
            context: self.context,
            body: self.body.linearize(context, used_vars),
        }
    }
}

impl Print for Def {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(DEF)
            .append(alloc.space())
            .append(&self.name)
            .append(self.context.print(cfg, alloc).parens())
            .append(alloc.space())
            .append(COLONEQ)
            .append(alloc.space())
            .append(self.body.print(cfg, alloc))
    }
}
