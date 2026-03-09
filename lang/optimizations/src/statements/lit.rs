use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState, Rename};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::{
    syntax::{
        names::{Identifier, fresh_identifier},
        statements::Literal,
    },
    traits::substitution::Subst,
};

use std::collections::HashSet;

impl Rewrite for Literal {
    type Target = Self;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        self.next = self.next.rewrite(state);
        self
    }
}

impl CleanupInlineGather for Literal {
    type Target = Self;
    fn cleanup_inline_gather(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.next = self.next.cleanup_inline_gather(state);
        self
    }
}

impl CleanupInline for Literal {
    type Target = Self;
    fn cleanup_inline(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.next = self.next.cleanup_inline(state);
        self
    }
}

impl Rename for Literal {
    fn rename(mut self, vars_to_rename: &HashSet<Identifier>, max_id: &mut usize) -> Self {
        if vars_to_rename.contains(&self.var) {
            let new_variable = fresh_identifier(max_id, &self.var.name);
            self.var = new_variable;

            self.next = self
                .next
                .subst_sim(&[(self.var.id, self.var.clone())])
                .rename(vars_to_rename, max_id);
        } else {
            self.next = self.next.rename(vars_to_rename, max_id);
        }

        self
    }
}
