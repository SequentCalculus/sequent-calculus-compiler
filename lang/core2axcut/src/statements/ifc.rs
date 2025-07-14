//! This module defines the translation for the conditionals comparing two variables.

use core_lang::syntax::statements::FsIfC;

use crate::shrinking::{Shrinking, ShrinkingState};

impl Shrinking for FsIfC {
    type Target = axcut::syntax::Statement;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::Statement {
        axcut::syntax::Statement::IfC(axcut::syntax::statements::IfC {
            sort: match self.sort {
                core_lang::syntax::statements::IfSort::Equal => {
                    axcut::syntax::statements::ifc::IfSort::Equal
                }
                core_lang::syntax::statements::IfSort::NotEqual => {
                    axcut::syntax::statements::ifc::IfSort::NotEqual
                }
                core_lang::syntax::statements::IfSort::Less => {
                    axcut::syntax::statements::ifc::IfSort::Less
                }
                core_lang::syntax::statements::IfSort::LessOrEqual => {
                    axcut::syntax::statements::ifc::IfSort::LessOrEqual
                }
                core_lang::syntax::statements::IfSort::Greater => {
                    axcut::syntax::statements::ifc::IfSort::Greater
                }
                core_lang::syntax::statements::IfSort::GreaterOrEqual => {
                    axcut::syntax::statements::ifc::IfSort::GreaterOrEqual
                }
            },
            fst: self.fst,
            snd: self.snd,
            thenc: self.thenc.shrink(state),
            elsec: self.elsec.shrink(state),
        })
    }
}
