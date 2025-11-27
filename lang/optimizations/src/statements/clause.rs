use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::statements::Clause;

impl Rewrite for Clause {
    type Target = Self;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        self.body = self.body.rewrite(state);
        self
    }
}
