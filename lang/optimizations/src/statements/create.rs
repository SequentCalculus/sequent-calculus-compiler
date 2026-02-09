use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState, Rename};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::{
    Var,
    names::fresh_name,
    statements::{Create, Statement},
};
use axcut::traits::{substitution::Subst, typed_free_vars::TypedFreeVars};

use std::{
    collections::{BTreeSet, HashSet},
    rc::Rc,
};

impl Rewrite for Create {
    type Target = Statement;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        let clauses = self
            .clauses
            .into_iter()
            .map(|clause| clause.rewrite(state))
            .collect();

        state.create_bindings.insert(self.var.clone(), clauses);
        self.next = self.next.rewrite(state);
        let clauses = state
            .create_bindings
            .remove(&self.var)
            .unwrap_or_else(|| panic!("Create variable {} not in bindings anymore", self.var));

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

impl Rename for Create {
    fn rename(mut self, vars_to_rename: &HashSet<Var>, used_vars: &mut HashSet<Var>) -> Self {
        let used_vars_clone = used_vars.clone();
        self.clauses = self
            .clauses
            .into_iter()
            .map(|clause| {
                let mut used_vars_clause = used_vars_clone.clone();
                let clause = clause.rename(vars_to_rename, &mut used_vars_clause);
                used_vars.extend(used_vars_clause);
                clause
            })
            .collect();

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
