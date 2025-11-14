use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::statements::Invoke;

impl Rewrite for Invoke {
    type Target = Self;
    fn rewrite(self, _: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(self)
    }
}
