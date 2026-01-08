use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::statements::Literal;

impl Rewrite for Literal {
    type Target = Self;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        self.next = self.next.rewrite(state);
        self
    }
}
