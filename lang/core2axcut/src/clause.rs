use core::syntax_var::{Clause, TypeDeclaration, Var};

use crate::traits::{Shrinking, UsedBinders};

use std::collections::HashSet;

impl UsedBinders for Clause {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        for binding in &self.context {
            used.insert(binding.var.clone());
        }
        self.case.used_binders(used);
    }
}

impl Shrinking for Clause {
    type Target = axcut::syntax::Clause;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[TypeDeclaration],
    ) -> axcut::syntax::Clause {
        axcut::syntax::Clause {
            xtor: self.xtor,
            context: self.context.shrink(used_vars, types),
            case: self.case.shrink(used_vars, types),
        }
    }
}
