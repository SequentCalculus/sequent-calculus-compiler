use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState, Rename};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::{
    syntax::{Var, names::fresh_name, statements::Op},
    traits::substitution::Subst,
};

use std::collections::HashSet;

impl Rewrite for Op {
    type Target = Self;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        self.next = self.next.rewrite(state);
        self
    }
}

impl CleanupInlineGather for Op {
    type Target = Self;
    fn cleanup_inline_gather(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.next = self.next.cleanup_inline_gather(state);
        self
    }
}

impl CleanupInline for Op {
    type Target = Self;
    fn cleanup_inline(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.next = self.next.cleanup_inline(state);
        self
    }
}

impl Rename for Op {
    fn rename(mut self, vars_to_rename: &HashSet<Var>, used_vars: &mut HashSet<Var>) -> Self {
        if vars_to_rename.contains(&self.var) {
            let new_variable = fresh_name(used_vars, &self.var);
            let old_variable = self.var;
            self.var = new_variable;

            self.next = self
                .next
                .subst_sim(&[(old_variable, self.var.clone())])
                .rename(vars_to_rename, used_vars);
        } else {
            self.next = self.next.rename(vars_to_rename, used_vars);
        }

        self
    }
}
