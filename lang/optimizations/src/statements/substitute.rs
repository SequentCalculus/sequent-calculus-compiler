use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState, Rename};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::{
    syntax::{
        ContextBinding,
        names::{ID, Identifier, fresh_identifier},
        statements::Substitute,
    },
    traits::substitution::Subst,
};

use std::collections::HashSet;

impl Rewrite for Substitute {
    type Target = Self;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        self.next = self.next.rewrite(state);
        self
    }
}

impl CleanupInlineGather for Substitute {
    type Target = Self;
    fn cleanup_inline_gather(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.next = self.next.cleanup_inline_gather(state);
        self
    }
}

impl CleanupInline for Substitute {
    type Target = Self;
    fn cleanup_inline(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.next = self.next.cleanup_inline(state);
        self
    }
}

impl Rename for Substitute {
    fn rename(mut self, vars_to_rename: &HashSet<Identifier>, max_id: &mut usize) -> Self {
        let mut new_rearrange = Vec::new();
        let mut subst: Vec<(ID, Identifier)> = Vec::new();

        for (binding, old) in self.rearrange {
            if vars_to_rename.contains(&binding.var) {
                let new_var: Identifier = fresh_identifier(max_id, &binding.var.name);
                new_rearrange.push((
                    ContextBinding {
                        var: new_var.clone(),
                        chi: binding.chi.clone(),
                        ty: binding.ty.clone(),
                    },
                    old,
                ));

                subst.push((binding.var.id, new_var));
            } else {
                new_rearrange.push((binding, old));
            }
        }

        self.rearrange = new_rearrange;

        self.next = if subst.is_empty() {
            self.next.rename(vars_to_rename, max_id)
        } else {
            self.next.subst_sim(&subst).rename(vars_to_rename, max_id)
        };

        self
    }
}
