//! This module defines the translation for a clause in a pattern or copattern match.

use core_lang::syntax::FsStatement;
use core_lang::syntax::terms::{Clause, PrdCns};

use crate::context::shrink_context;
use crate::shrinking::{Shrinking, ShrinkingState};

impl<T: PrdCns> Shrinking for Clause<T, FsStatement> {
    type Target = axcut::syntax::statements::Clause;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::statements::Clause {
        axcut::syntax::statements::Clause {
            xtor: self.xtor,
            context: shrink_context(self.context, state.codata),
            body: self.body.shrink(state),
        }
    }
}
