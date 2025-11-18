//! This module defines the translation for the call of a top-level function.

use core_lang::syntax::statements::FsCall;

use crate::{
    context::shrink_context,
    shrinking::{Shrinking, ShrinkingState},
};

impl Shrinking for FsCall {
    type Target = axcut::syntax::Statement;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::Statement {
        axcut::syntax::Statement::Call(axcut::syntax::statements::Call {
            label: self.name,
            context: shrink_context(self.args, state.codata),
        })
    }
}
