use super::{cont_int, stringify_and_join, Def, TypeDeclaration};
use crate::traits::shrink::{Shrinking, UsedBinders};

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
        write!(f, "{types_joined}\n\n{defs_joined}")
    }
}

impl Prog {
    #[must_use]
    pub fn shrink(mut self) -> axcut::syntax::Prog {
        let cont_int = cont_int();
        for typ in &self.types {
            assert!(
                typ.name != cont_int.name,
                "{} cannot be used as a type name",
                cont_int.name
            );
        }
        self.types.push(cont_int);
        axcut::syntax::Prog {
            defs: self
                .defs
                .into_iter()
                .map(|def| {
                    let mut used_vars = HashSet::new();
                    def.body.used_binders(&mut used_vars);
                    for binding in &def.context {
                        used_vars.insert(binding.var.clone());
                    }
                    def.shrink(&mut used_vars, &self.types)
                })
                .collect(),
            types: self
                .types
                .shrink(&mut HashSet::default(), Default::default()),
        }
    }
}
