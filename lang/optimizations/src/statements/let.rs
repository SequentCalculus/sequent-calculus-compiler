use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState, Rename};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::{
    Var,
    names::fresh_var,
    statements::{Let, Statement},
};
use axcut::traits::{substitution::Subst, typed_free_vars::TypedFreeVars};

use std::{
    collections::{BTreeSet, HashSet},
    rc::Rc,
};

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
            *state.new_changes = true;
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

impl Rename for Let {
    fn rename(mut self, vars_to_rename: &HashSet<Var>, used_vars: &mut HashSet<Var>) -> Self {
        if vars_to_rename.contains(&self.var) {
            let new_variable = fresh_var(used_vars, &self.var.name);
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
