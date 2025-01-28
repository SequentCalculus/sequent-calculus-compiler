use core_lang::syntax::statement::FsPrintLnI64;

use crate::traits::{Shrinking, ShrinkingState};

impl Shrinking for FsPrintLnI64 {
    type Target = axcut::syntax::Statement;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::Statement {
        axcut::syntax::Statement::PrintLnI64(axcut::syntax::statements::PrintLnI64 {
            var: self.var,
            next: self.next.shrink(state),
        })
    }
}
