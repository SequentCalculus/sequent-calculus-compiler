use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState, Rename};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::{Var, statements::IfC};

use std::collections::HashSet;

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

impl Rename for IfC {
    fn rename(mut self, vars_to_rename: &HashSet<Var>, used_vars: &mut HashSet<Var>) -> Self {
        let mut used_vars_thenc = used_vars.clone();
        self.thenc = self.thenc.rename(vars_to_rename, &mut used_vars_thenc);
        self.elsec = self.elsec.rename(vars_to_rename, used_vars);
        used_vars.extend(used_vars_thenc);

        self
    }
}
