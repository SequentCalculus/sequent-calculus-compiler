use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::statements::{Create, Statement};
use axcut::traits::typed_free_vars::TypedFreeVars;
use std::{collections::BTreeSet, rc::Rc};

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
        let clauses = state.create_bindings.remove(&self.var).unwrap();

        let mut free_vars = BTreeSet::new();
        self.next.typed_free_vars(&mut free_vars);
        if free_vars.iter().all(|binding| binding.var != self.var) {
            *state.new_changes = true;
            Rc::unwrap_or_clone(self.next)
        } else {
            self.clauses = clauses;
            self.into()
        }
    }
}
