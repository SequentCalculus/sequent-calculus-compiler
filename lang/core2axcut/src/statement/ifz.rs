use core_lang::syntax::statement::FsIfZ;

use crate::traits::{Shrinking, ShrinkingState};

impl Shrinking for FsIfZ {
    type Target = axcut::syntax::Statement;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::Statement {
        axcut::syntax::Statement::IfZ(axcut::syntax::statements::IfZ {
            sort: axcut::syntax::statements::ifz::IfZSort::Equal,
            ifc: self.ifc,
            thenc: self.thenc.shrink(state),
            elsec: self.elsec.shrink(state),
        })
    }
}
