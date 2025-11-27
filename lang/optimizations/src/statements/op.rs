use crate::{
    errors::Error,
    rewrite::{Rewrite, RewriteState},
};
use axcut::syntax::statements::Op;

impl Rewrite for Op {
    type Target = Self;
    fn rewrite(mut self, state: &mut RewriteState) -> Result<Self::Target, Error> {
        self.next = self.next.rewrite(state)?;
        Ok(self)
    }
}
