use printer::{theme::ThemeExt, tokens::TYPE, util::BracesExt, DocAllocator, Print};

use super::{Name, TypingContext};

#[derive(Debug, Clone)]
pub struct XtorSig {
    pub name: Name,
    pub args: TypingContext,
}

#[derive(Debug, Clone)]
pub struct TypeDeclaration {
    pub name: Name,
    pub xtors: Vec<XtorSig>,
}

impl Print for XtorSig {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(&self.name).append(self.args.print(cfg, alloc))
    }
}

impl Print for TypeDeclaration {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(TYPE)
            .append(alloc.space())
            .append(alloc.typ(&self.name))
            .append(alloc.space())
            .append(self.xtors.print(cfg, alloc).braces_anno())
    }
}

impl TypeDeclaration {
    pub fn xtor_position(&self, tag: &Name) -> usize {
        self.xtors
            .iter()
            .position(|xtor| xtor.name == *tag)
            .unwrap_or_else(|| {
                panic!(
                    "Constructor {tag} not found in type declaration {}",
                    self.print_to_string(None)
                )
            })
    }
}
