use core::syntax_var::{statement::FsIfZ, FsTypeDeclaration, Var};

use crate::traits::Shrinking;

use std::collections::HashSet;

impl Shrinking for FsIfZ {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[FsTypeDeclaration],
    ) -> axcut::syntax::Statement {
        axcut::syntax::Statement::IfZ(axcut::syntax::statements::IfZ {
            ifc: self.ifc,
            thenc: self.thenc.shrink(used_vars, types),
            elsec: self.elsec.shrink(used_vars, types),
        })
    }
}
