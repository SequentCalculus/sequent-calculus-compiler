use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::statements::Clause;

impl Rewrite for Clause {
    type Target = Self;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        self.body = self.body.rewrite(state);
        self
    }
}

impl CleanupInlineGather for Clause {
    type Target = Self;
    fn cleanup_inline_gather(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.body = self.body.cleanup_inline_gather(state);
        self
    }
}

impl CleanupInline for Clause {
    type Target = Self;
    fn cleanup_inline(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.body = self.body.cleanup_inline(state);
        self
    }
}
