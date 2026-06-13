//! This module defines programs in AxCut.

use printer::{DocAllocator, Print};

use crate::syntax::{Identifier, XtorSig};
use crate::traits::steadfast::{
    CollectSteadfastConstraints, SteadfastConstraints, SubstituteSteadfastAnnotations,
};

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
    /// This function applies the linearization procedure to the whole program, which means to all
    /// top-level functions.
    pub fn linearize(&mut self) {
        let new_defs = Vec::with_capacity(self.defs.len());
        for mut def in std::mem::replace(&mut self.defs, new_defs) {
            def = def.linearize(&mut self.max_id);
            self.defs.push(def);
        }

        self.infer_quantities();
    }

    pub fn infer_quantities(&mut self) {
        let mut constraints = SteadfastConstraints::default();

        for def in &self.defs {
            def.collect_sf_constraints(self, &[], &mut constraints);
        }

        let solution = constraints.resolve();

        let new_types = Vec::with_capacity(self.types.len());
        for mut typ in std::mem::replace(&mut self.types, new_types) {
            typ = typ.subst_sf_annotations(&solution);
            self.types.push(typ);
        }

        let new_defs = Vec::with_capacity(self.defs.len());
        for mut def in std::mem::replace(&mut self.defs, new_defs) {
            def = def.subst_sf_annotations(&solution);
            self.defs.push(def);
        }
    }

    pub fn find_def(&self, name: &Identifier) -> Option<&Def> {
        self.defs.iter().find(|def| def.name == *name)
    }

    pub fn find_xtor(&self, type_name: &Identifier, xtor_name: &Identifier) -> Option<&XtorSig> {
        self.types
            .iter()
            .find(|types| types.name == *type_name)
            .and_then(|ty| ty.xtors.iter().find(|xtor| xtor.name == *xtor_name))
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
