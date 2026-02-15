//! This module defines the translation for a clause in a pattern or copattern match.

use core_lang::syntax::FsStatement;
use core_lang::syntax::terms::{Chi, Clause};

use crate::context::shrink_context;
use crate::shrink_ident;
use crate::shrinking::{Shrinking, ShrinkingState};

impl<T: Chi> Shrinking for Clause<T, FsStatement> {
    type Target = axcut::syntax::statements::Clause;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::statements::Clause {
        axcut::syntax::statements::Clause {
            xtor: shrink_ident(self.xtor),
            context: shrink_context(self.context, state.codata),
            body: self.body.shrink(state),
        }
    }
}
