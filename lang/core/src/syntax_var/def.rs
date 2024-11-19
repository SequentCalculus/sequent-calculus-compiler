use printer::{
    theme::ThemeExt,
    tokens::{COLONEQ, DEF, SEMI},
    DocAllocator, Print,
};

use super::{Name, Statement, TypingContext};

#[derive(Debug, Clone, PartialEq)]
pub struct Def {
    pub name: Name,
    pub context: TypingContext,
    pub body: Statement,
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
            .append(alloc.text(&self.name))
            .append(self.context.print(cfg, alloc).parens())
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
