use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::statements::IfC;

impl Rewrite for IfC {
    type Target = Self;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        self.thenc = self.thenc.rewrite(state);
        self.elsec = self.elsec.rewrite(state);
        self
    }
}

impl CleanupInlineGather for IfC {
    type Target = Self;
    fn cleanup_inline_gather(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.thenc = self.thenc.cleanup_inline_gather(state);
        self.elsec = self.elsec.cleanup_inline_gather(state);
        self
    }
}

impl CleanupInline for IfC {
    type Target = Self;
    fn cleanup_inline(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.thenc = self.thenc.cleanup_inline(state);
        self.elsec = self.elsec.cleanup_inline(state);
        self
    }
}
