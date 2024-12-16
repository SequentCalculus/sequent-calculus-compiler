use core_lang::syntax::statement::FsCall;

use crate::traits::{Shrinking, ShrinkingState};

impl Shrinking for FsCall {
    type Target = axcut::syntax::Statement;

    fn shrink(self, _state: &mut ShrinkingState) -> axcut::syntax::Statement {
        axcut::syntax::Statement::Call(axcut::syntax::statements::Call {
            label: self.name,
            args: self.args,
        })
    }
}
