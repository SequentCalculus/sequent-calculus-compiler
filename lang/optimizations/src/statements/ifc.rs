use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::statements::IfC;

impl Rewrite for IfC {
    type Target = Self;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        self.thenc = self.thenc.rewrite(state);
        self.elsec = self.elsec.rewrite(state);
        self
    }
}
