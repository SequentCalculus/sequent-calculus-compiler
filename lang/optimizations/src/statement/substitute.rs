use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::statements::Substitute;

impl Rewrite for Substitute {
    type Target = Self;
    fn rewrite(self, _: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(self)
    }
}
