use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState, Rename};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::{
    syntax::{ContextBinding, Var, names::fresh_var, statements::Clause},
    traits::substitution::Subst,
};

use std::collections::HashSet;

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

impl Rename for Clause {
    fn rename(mut self, vars_to_rename: &HashSet<Var>, used_vars: &mut HashSet<Var>) -> Self {
        let mut new_bindings = Vec::new();
        let mut subst: Vec<(Var, Var)> = Vec::new();

        for binding in self.context.bindings {
            if vars_to_rename.contains(&binding.var) {
                let new_var: Var = fresh_var(used_vars, &binding.var.name);
                new_bindings.push(ContextBinding {
                    var: new_var.clone(),
                    chi: binding.chi.clone(),
                    ty: binding.ty.clone(),
                });

                subst.push((binding.var, new_var));
            } else {
                new_bindings.push(binding);
            }
        }

        self.context = new_bindings.into();

        self.body = if subst.is_empty() {
            self.body.rename(vars_to_rename, used_vars)
        } else {
            self.body
                .subst_sim(&subst)
                .rename(vars_to_rename, used_vars)
        };

        self
    }
}
