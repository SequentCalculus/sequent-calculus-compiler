use core::syntax_var::{statement::FsIfL, FsTypeDeclaration, Var};

use crate::traits::Shrinking;

use std::collections::HashSet;

impl Shrinking for FsIfL {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[FsTypeDeclaration],
    ) -> axcut::syntax::Statement {
        axcut::syntax::Statement::IfL(axcut::syntax::statements::IfL {
            fst: self.fst,
            snd: self.snd,
            thenc: self.thenc.shrink(used_vars, types),
            elsec: self.elsec.shrink(used_vars, types),
        })
    }
}
