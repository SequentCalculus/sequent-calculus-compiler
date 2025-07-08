//! This module defines the translation for printing an integer.

use core_lang::syntax::statements::FsPrintI64;

use crate::shrinking::{Shrinking, ShrinkingState};

impl Shrinking for FsPrintI64 {
    type Target = axcut::syntax::Statement;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::Statement {
        axcut::syntax::Statement::PrintI64(axcut::syntax::statements::PrintI64 {
            newline: self.newline,
            var: self.var,
            next: self.next.shrink(state),
            free_vars_next: None,
        })
    }
}
