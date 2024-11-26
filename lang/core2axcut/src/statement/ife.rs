use core::syntax::{declaration::FsTypeDeclaration, statement::FsIfC, Var};

use crate::traits::Shrinking;

use std::collections::HashSet;

impl Shrinking for FsIfC {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[FsTypeDeclaration],
    ) -> axcut::syntax::Statement {
        axcut::syntax::Statement::IfC(axcut::syntax::statements::IfC {
            sort: match self.sort {
                core::syntax::statement::IfSort::Equal => {
                    axcut::syntax::statements::ifc::IfSort::Equal
                }
                core::syntax::statement::IfSort::Less => {
                    axcut::syntax::statements::ifc::IfSort::Less
                }
            },
            fst: self.fst,
            snd: self.snd,
            thenc: self.thenc.shrink(used_vars, types),
            elsec: self.elsec.shrink(used_vars, types),
        })
    }
}
