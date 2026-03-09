use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState, Rename};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::{
    syntax::{
        names::Identifier,
        statements::{Statement, Switch},
    },
    traits::substitution::Subst,
};

use std::{collections::HashSet, rc::Rc};

impl Rewrite for Switch {
    type Target = Statement;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        match state.get_let(&self.var) {
            None => {
                self.clauses = self
                    .clauses
                    .into_iter()
                    .map(|clause| clause.rewrite(state))
                    .collect();
                self.into()
            }
            Some((xtor, args)) => {
                state.new_changes = true;

                let clause = self
                    .clauses
                    .into_iter()
                    .find(|clause| clause.xtor == xtor)
                    .unwrap_or_else(|| panic!("Could not find switch clause binding for {xtor}"));
                let subst = clause
                    .context
                    .into_iter_vars()
                    .map(|var| var.id)
                    .zip(args.into_iter_vars())
                    .collect::<Vec<_>>();

                Rc::unwrap_or_clone(clause.body.subst_sim(&subst).rewrite(state))
            }
        }
    }
}

impl CleanupInlineGather for Switch {
    type Target = Self;
    fn cleanup_inline_gather(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.clauses = self.clauses.cleanup_inline_gather(state);
        self
    }
}

impl CleanupInline for Switch {
    type Target = Self;
    fn cleanup_inline(mut self, state: &mut CleanupInlineState) -> Self::Target {
        self.clauses = self.clauses.cleanup_inline(state);
        self
    }
}

impl Rename for Switch {
    fn rename(mut self, vars_to_rename: &HashSet<Identifier>, max_id: &mut usize) -> Self {
        self.clauses = self
            .clauses
            .into_iter()
            .map(|clause| {
                let clause = clause.rename(vars_to_rename, max_id);
                clause
            })
            .collect();

        self
    }
}
