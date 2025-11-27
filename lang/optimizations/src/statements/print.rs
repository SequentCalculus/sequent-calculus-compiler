use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::statements::PrintI64;

impl Rewrite for PrintI64 {
    type Target = Self;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        self.next = self.next.rewrite(state);
        self
    }
}
