use printer::{DocAllocator, Print};

use crate::traits::*;

use super::{
    declaration::{CodataDeclaration, DataDeclaration},
    def::FsDef,
    Def,
};

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XProg<T> {
    pub defs: Vec<T>,
    pub data_types: Vec<DataDeclaration>,
    pub codata_types: Vec<CodataDeclaration>,
}

pub type Prog = XProg<Def>;
pub type FsProg = XProg<FsDef>;

impl<T: Print> Print for XProg<T> {
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
        let data_types = self.data_types.iter().map(|typ| typ.print(cfg, alloc));
        let codata_types = self.codata_types.iter().map(|typ| typ.print(cfg, alloc));

        alloc
            .intersperse(data_types, alloc.line())
            .append(alloc.line())
            .append(alloc.intersperse(codata_types, alloc.line()))
            .append(sep.clone())
            .append(alloc.intersperse(defs, sep))
    }
}

#[must_use]
pub fn transform_prog(prog: Prog) -> FsProg {
    let codata_types_clone = prog.codata_types.clone();
    let mut state = FocusingState {
        codata_types: codata_types_clone.as_slice(),
        ..FocusingState::default()
    };

    FsProg {
        defs: prog
            .defs
            .into_iter()
            .map(|mut def| {
                let mut used_vars = HashSet::new();
                def.body.used_binders(&mut used_vars);
                used_vars.extend(def.context.vars());
                used_vars.extend(def.context.covars());

                let mut seen_vars = def.context.vars();
                seen_vars.extend(def.context.covars());

                def.body = def.body.uniquify(&mut seen_vars, &mut used_vars);

                state.used_vars = used_vars;
                def.focus(&mut state)
            })
            .collect(),
        data_types: prog.data_types,
        codata_types: prog.codata_types,
    }
}
