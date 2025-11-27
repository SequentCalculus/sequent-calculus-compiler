use crate::{
    errors::Error,
    rewrite::{Rewrite, RewriteState},
};
use axcut::syntax::statements::{Let, Statement};
use axcut::traits::typed_free_vars::TypedFreeVars;

use std::{collections::BTreeSet, rc::Rc};

impl Rewrite for Let {
    type Target = Statement;
    fn rewrite(mut self, state: &mut RewriteState) -> Result<Self::Target, Error> {
        state
            .let_bindings
            .insert(self.var.clone(), (self.tag, self.args));
        self.next = self.next.rewrite(state)?;
        let (tag, args) = state.let_bindings.remove(&self.var).unwrap();

        let mut free_vars = BTreeSet::new();
        self.next.typed_free_vars(&mut free_vars);
        if free_vars.iter().all(|binding| binding.var != self.var) {
            *state.new_changes = true;
            Ok(Rc::unwrap_or_clone(self.next))
        } else {
            self.tag = tag;
            self.args = args;
            Ok(self.into())
        }
    }
}
