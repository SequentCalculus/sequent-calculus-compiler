use core::syntax_var::{Def, TypeDeclaration, Var};

use crate::context::translate_context;
use crate::traits::Shrinking;

use std::collections::HashSet;

impl Shrinking for Def {
    type Target = axcut::syntax::Def;

    fn shrink(
        mut self,
        _used_vars: &mut HashSet<Var>,
        types: &[TypeDeclaration],
    ) -> axcut::syntax::Def {
        axcut::syntax::Def {
            name: self.name,
            context: translate_context(self.context),
            body: self.body.shrink(&mut self.used_vars, types),
            used_vars: self.used_vars,
        }
    }
}
