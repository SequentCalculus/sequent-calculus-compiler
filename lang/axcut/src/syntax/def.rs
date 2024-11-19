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
}

impl Linearizing for Def {
    type Target = Def;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Def {
        Def {
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
