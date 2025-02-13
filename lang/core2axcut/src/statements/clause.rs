use core_lang::syntax::terms::{Clause, PrdCns};
use core_lang::syntax::FsStatement;

use crate::context::translate_context;
use crate::shrinking::{Shrinking, ShrinkingState};

impl<T: PrdCns> Shrinking for Clause<T, FsStatement> {
    type Target = axcut::syntax::statements::Clause;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::statements::Clause {
        axcut::syntax::statements::Clause {
            xtor: self.xtor,
            context: translate_context(self.context, state.codata),
            case: self.rhs.shrink(state),
        }
    }
}
