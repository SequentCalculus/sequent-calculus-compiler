use core::syntax_var::{Def, TypeDeclaration, Var};

use crate::traits::Shrinking;

use std::collections::HashSet;

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
