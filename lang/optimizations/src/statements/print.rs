use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState, Rename};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::{names::Identifier, statements::PrintI64};

use std::collections::HashSet;

impl Rewrite for PrintI64 {
    type Target = Self;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        self.next = self.next.rewrite(state);
        self
    }
}

impl CleanupInlineGather for PrintI64 {
    type Target = Self;
    fn cleanup_inline_gather(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.next = self.next.cleanup_inline_gather(state);
        self
    }
}

impl CleanupInline for PrintI64 {
    type Target = Self;
    fn cleanup_inline(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.next = self.next.cleanup_inline(state);
        self
    }
}

impl Rename for PrintI64 {
    fn rename(mut self, vars_to_rename: &HashSet<Identifier>, max_id: &mut usize) -> Self {
        self.next = self.next.rename(vars_to_rename, max_id);
        self
    }
}
