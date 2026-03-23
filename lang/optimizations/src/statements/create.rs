use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::statements::{Create, Statement};
use axcut::traits::typed_free_vars::TypedFreeVars;
use printer::Print;

use std::{collections::BTreeSet, rc::Rc};

impl Rewrite for Create {
    type Target = Statement;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        let clauses = self
            .clauses
            .into_iter()
            .map(|clause| clause.rewrite(state))
            .collect();

        state.create_bindings.insert(self.var.id, clauses);
        self.next = self.next.rewrite(state);
        let clauses = state
            .create_bindings
            .remove(&self.var.id)
            .unwrap_or_else(|| {
                panic!(
                    "Create variable {} not in bindings anymore",
                    self.var.print_to_string(None)
                )
            });

        let mut free_vars = BTreeSet::new();
        self.next.typed_free_vars(&mut free_vars);
        if free_vars.iter().all(|binding| binding.var != self.var) {
            state.new_changes = true;
            Rc::unwrap_or_clone(self.next)
        } else {
            self.clauses = clauses;
            self.into()
        }
    }
}

impl CleanupInlineGather for Create {
    type Target = Self;
    fn cleanup_inline_gather(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.clauses = self.clauses.cleanup_inline_gather(state);
        self.next = self.next.cleanup_inline_gather(state);
        self
    }
}

impl CleanupInline for Create {
    type Target = Self;
    fn cleanup_inline(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.clauses = self.clauses.cleanup_inline(state);
        self.next = self.next.cleanup_inline(state);
        self
    }
}
