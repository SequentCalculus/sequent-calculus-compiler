use super::Def;
use crate::syntax::{stringify_and_join, TypeDeclaration};
use crate::traits::linearize::{Linearizing, UsedBinders};

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Prog {
    pub defs: Vec<Def>,
    pub types: Vec<TypeDeclaration>,
}

impl fmt::Display for Prog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let types_joined: String = stringify_and_join(&self.types, "\n");
        let defs_joined: String = stringify_and_join(&self.defs, "\n\n");
        write!(f, "{defs_joined}\n{types_joined}")
    }
}

pub fn linearize(program: Prog) -> crate::syntax::Prog {
    crate::syntax::Prog {
        defs: program
            .defs
            .into_iter()
            .map(|def| {
                let context = def.context.clone();
                let mut used_vars = HashSet::new();
                def.body.used_binders(&mut used_vars);
                for binding in &context {
                    used_vars.insert(binding.var.clone());
                }
                def.linearize(context, &mut used_vars)
            })
            .collect(),
        types: program.types,
    }
}
