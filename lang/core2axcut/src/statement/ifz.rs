use core::syntax_var::{statement::IfZ, TypeDeclaration, Var};

use crate::traits::{Shrinking, UsedBinders};

use std::collections::HashSet;

impl UsedBinders for IfZ {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.thenc.used_binders(used);
        self.elsec.used_binders(used);
    }
}

impl Shrinking for IfZ {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[TypeDeclaration],
    ) -> axcut::syntax::Statement {
        axcut::syntax::Statement::IfZ(axcut::syntax::IfZ {
            ifc: self.ifc,
            thenc: self.thenc.shrink(used_vars, types),
            elsec: self.elsec.shrink(used_vars, types),
        })
    }
}
