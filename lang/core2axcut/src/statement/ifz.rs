use core_lang::syntax::statement::FsIfZ;

use crate::traits::{Shrinking, ShrinkingState};

impl Shrinking for FsIfZ {
    type Target = axcut::syntax::Statement;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::Statement {
        axcut::syntax::Statement::IfZ(axcut::syntax::statements::IfZ {
            sort: match self.sort {
                core_lang::syntax::statement::IfZSort::Equal => {
                    axcut::syntax::statements::ifz::IfZSort::Equal
                }
                core_lang::syntax::statement::IfZSort::NotEqual => {
                    axcut::syntax::statements::ifz::IfZSort::NotEqual
                }
            },
            ifc: self.ifc,
            thenc: self.thenc.shrink(state),
            elsec: self.elsec.shrink(state),
        })
    }
}
