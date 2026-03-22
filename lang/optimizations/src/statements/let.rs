use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::statements::{Let, Statement};
use axcut::traits::typed_free_vars::TypedFreeVars;

use std::{collections::BTreeSet, rc::Rc};

impl Rewrite for Let {
    type Target = Statement;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        state
            .let_bindings
            .insert(self.var.clone(), (self.tag, self.args));
        self.next = self.next.rewrite(state);
        let (tag, args) = state.let_bindings.remove(&self.var).unwrap();

        let mut free_vars = BTreeSet::new();
        self.next.typed_free_vars(&mut free_vars);
        if free_vars.iter().all(|binding| binding.var != self.var) {
            state.new_changes = true;
            Rc::unwrap_or_clone(self.next)
        } else {
            self.tag = tag;
            self.args = args;
            self.into()
        }
    }
}

impl CleanupInlineGather for Let {
    type Target = Self;
    fn cleanup_inline_gather(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.next = self.next.cleanup_inline_gather(state);
        self
    }
}

impl CleanupInline for Let {
    type Target = Self;
    fn cleanup_inline(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.next = self.next.cleanup_inline(state);
        self
    }
}
