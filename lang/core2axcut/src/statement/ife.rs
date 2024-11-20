use core::syntax_var::{statement::IfE, TypeDeclaration, Var};

use crate::traits::Shrinking;

use std::collections::HashSet;

impl Shrinking for IfE {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[TypeDeclaration],
    ) -> axcut::syntax::Statement {
        axcut::syntax::Statement::IfE(axcut::syntax::statements::IfE {
            fst: self.fst,
            snd: self.snd,
            thenc: self.thenc.shrink(used_vars, types),
            elsec: self.elsec.shrink(used_vars, types),
        })
    }
}
