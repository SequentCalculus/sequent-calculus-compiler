//! This module defines the translation for the unreachable statement.

use crate::shrinking::{Shrinking, ShrinkingState};
use core_lang::syntax::statements::FsUnreachable;

impl Shrinking for FsUnreachable {
    type Target = axcut::syntax::Statement;

    fn shrink(self, _state: &mut ShrinkingState) -> axcut::syntax::Statement {
        axcut::syntax::Statement::Unreachable(axcut::syntax::statements::Unreachable)
    }
}
