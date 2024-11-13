use super::{stringify_and_join, Name, Statement, TypeDeclaration, TypingContext, Var};
use crate::traits::shrink::Shrinking;

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Def {
    pub name: Name,
    pub context: TypingContext,
    pub body: Statement,
}

impl std::fmt::Display for Def {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = stringify_and_join(&self.context, ", ");
        write!(f, "def {}({}) :=\n  {}", self.name, args, self.body)
    }
}

impl Shrinking for Def {
    type Target = axcut::syntax::Def;

    fn shrink(self, used_vars: &mut HashSet<Var>, types: &[TypeDeclaration]) -> axcut::syntax::Def {
        axcut::syntax::Def {
            name: self.name,
            context: self.context.shrink(used_vars, types),
            body: self.body.shrink(used_vars, types),
        }
    }
}
