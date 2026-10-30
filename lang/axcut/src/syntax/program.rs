//! This module defines programs in AxCut.

use std::collections::HashMap;
use printer::{DocAllocator, Print};
use crate::syntax::statements::count_used_functions;
use super::{Def, ID, TypeDeclaration};

/// This struct defines programs in AxCut. They consist of a list top-level functions and a list of
/// user-declared types. Moreover, it contains the highest [`ID`] currently used for
/// [`crate::syntax::Identifier`]s in the program.
#[derive(Debug, Clone, PartialEq)]
pub struct Prog {
    pub defs: Vec<Def>,
    pub types: Vec<TypeDeclaration>,
    pub max_id: ID,
}

impl Prog {
    pub fn inline(&mut self) {
        let mut occurences = self.count_occurences();
        self.inline_defs(&mut occurences);
        self.remove_unused(&occurences);
    }

    fn count_occurences(&self) -> HashMap<ID, i32> {
        let mut occurences: HashMap<ID, i32> = HashMap::with_capacity(self.defs.len());
        for def in self.defs.iter() {
            count_used_functions(&def.body, &mut occurences);
        }
        occurences
    }

    fn inline_defs(&mut self, occurences: &mut HashMap<ID, i32>) {
        for def in self.defs.iter_mut() {
            def.inline_defs(occurences);
        }
    }

    fn remove_unused(&mut self, occurences: &HashMap<ID, i32>) {
        unimplemented!()
    }

    /// This function applies the linearization procedure to the whole program, which means to all
    /// top-level functions.
    pub fn linearize(&mut self) {
        let new_defs = Vec::with_capacity(self.defs.len());
        for mut def in std::mem::replace(&mut self.defs, new_defs) {
            def = def.linearize(&mut self.max_id);
            self.defs.push(def);
        }
    }
}

impl Print for Prog {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        // We usually separate declarations with an empty line, except when the `omit_decl_sep`
        // option is set. This is useful for typesetting examples in papers which have to make
        // economic use of vertical space.
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
