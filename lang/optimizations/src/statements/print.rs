use crate::{
    errors::Error,
    rewrite::{Rewrite, RewriteState},
};
use axcut::syntax::statements::PrintI64;

impl Rewrite for PrintI64 {
    type Target = Self;
    fn rewrite(mut self, state: &mut RewriteState) -> Result<Self::Target, Error> {
        self.next = self.next.rewrite(state)?;
        Ok(self)
    }
}
