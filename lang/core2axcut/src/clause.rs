use core_lang::syntax::term::Clause;
use core_lang::syntax::FsStatement;

use crate::context::translate_context;
use crate::traits::{Shrinking, ShrinkingState};

impl Shrinking for Clause<FsStatement> {
    type Target = axcut::syntax::Clause;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::Clause {
        axcut::syntax::Clause {
            xtor: self.xtor,
            context: translate_context(self.context, state.codata),
            case: self.rhs.shrink(state),
        }
    }
}
