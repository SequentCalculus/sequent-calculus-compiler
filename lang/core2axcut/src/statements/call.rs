use core_lang::syntax::statements::FsCall;

use crate::shrinking::{Shrinking, ShrinkingState};

impl Shrinking for FsCall {
    type Target = axcut::syntax::Statement;

    fn shrink(self, _state: &mut ShrinkingState) -> axcut::syntax::Statement {
        axcut::syntax::Statement::Call(axcut::syntax::statements::Call {
            label: self.name,
            args: self.args,
        })
    }
}
