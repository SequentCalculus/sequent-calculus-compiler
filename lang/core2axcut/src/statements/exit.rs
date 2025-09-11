//! This module defines the translation for the exit statement.

use core_lang::syntax::statements::FsExit;

use crate::shrinking::{Shrinking, ShrinkingState};

impl Shrinking for FsExit {
    type Target = axcut::syntax::Statement;

    fn shrink(self, _state: &mut ShrinkingState) -> axcut::syntax::Statement {
        axcut::syntax::Statement::Exit(axcut::syntax::statements::Exit { var: self.var })
    }
}
