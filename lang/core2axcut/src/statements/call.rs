//! This module defines the translation for the call of a top-level function.

use core_lang::syntax::statements::FsCall;

use crate::{
    context::shrink_context,
    shrink_ident,
    shrinking::{Shrinking, ShrinkingState},
};

impl Shrinking for FsCall {
    type Target = axcut::syntax::Statement;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::Statement {
        axcut::syntax::Statement::Call(axcut::syntax::statements::Call {
            label: shrink_ident(self.name),
            args: shrink_context(self.args, state.codata),
        })
    }
}
