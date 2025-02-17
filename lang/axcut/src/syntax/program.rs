use printer::{DocAllocator, Print};

use super::{Def, TypeDeclaration};

#[derive(Debug, Clone)]
pub struct Prog {
    pub defs: Vec<Def>,
    pub types: Vec<TypeDeclaration>,
}

impl Prog {
    pub fn linearize(mut self) -> Prog {
        self.defs = self.defs.into_iter().map(Def::linearize).collect();
        self
    }
}

impl Print for Prog {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        // We usually separate declarations with an empty line, except when the `omit_decl_sep` option is set.
        // This is useful for typesetting examples in papers which have to make economic use of vertical space.
        let sep = if cfg.omit_decl_sep {
            alloc.line()
        } else {
            alloc.line().append(alloc.line())
        };

        let types = self.types.iter().map(|typ| typ.print(cfg, alloc));
        let defs = self.defs.iter().map(|def| def.print(cfg, alloc));

        alloc
            .intersperse(types, alloc.line())
            .append(sep.clone())
            .append(alloc.intersperse(defs, sep))
    }
}
