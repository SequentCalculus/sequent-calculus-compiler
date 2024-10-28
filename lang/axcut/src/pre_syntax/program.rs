use super::Def;
use crate::syntax::{context::context_vars, stringify_and_join, TypeDeclaration};
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

#[must_use]
pub fn linearize(program: Prog) -> crate::syntax::Prog {
    let mut used_vars = HashSet::new();
    for typ in &program.types {
        used_vars.insert(typ.name.clone());
    }
    for def in &program.defs {
        used_vars.insert(def.name.clone());
    }
    crate::syntax::Prog {
        defs: program
            .defs
            .into_iter()
            .map(|def| {
                let context = context_vars(&def.context);
                let mut used_vars = used_vars.clone();
                def.body.used_binders(&mut used_vars);
                for var in &context {
                    used_vars.insert(var.clone());
                }
                def.linearize(context, &mut used_vars)
            })
            .collect(),
        types: program.types,
    }
}
