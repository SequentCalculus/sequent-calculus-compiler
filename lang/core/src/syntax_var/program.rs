use printer::{DocAllocator, Print};

use crate::syntax::{declaration::FsTypeDeclaration, def::FsDef};

#[derive(Debug, Clone)]
pub struct FsProg {
    pub defs: Vec<FsDef>,
    pub types: Vec<FsTypeDeclaration>,
}

impl Print for FsProg {
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

        let defs = self.defs.iter().map(|def| def.print(cfg, alloc));
        let types = self.types.iter().map(|typ| typ.print(cfg, alloc));

        alloc
            .intersperse(types, alloc.line())
            .append(sep.clone())
            .append(alloc.intersperse(defs, sep))
    }
}
