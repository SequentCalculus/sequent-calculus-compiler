use core_lang::syntax::statement::FsIfC;

use crate::traits::{Shrinking, ShrinkingState};

impl Shrinking for FsIfC {
    type Target = axcut::syntax::Statement;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::Statement {
        axcut::syntax::Statement::IfC(axcut::syntax::statements::IfC {
            sort: match self.sort {
                core_lang::syntax::statement::IfSort::Equal => {
                    axcut::syntax::statements::ifc::IfSort::Equal
                }
                core_lang::syntax::statement::IfSort::Less => {
                    axcut::syntax::statements::ifc::IfSort::Less
                }
                core_lang::syntax::statement::IfSort::LessOrEqual => {
                    axcut::syntax::statements::ifc::IfSort::LessOrEqual
                }
            },
            fst: self.fst,
            snd: self.snd,
            thenc: self.thenc.shrink(state),
            elsec: self.elsec.shrink(state),
        })
    }
}
