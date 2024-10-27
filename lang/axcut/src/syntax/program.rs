use super::{stringify_and_join, Def, TypeDeclaration};

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
