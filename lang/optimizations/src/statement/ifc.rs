use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::statements::IfC;

impl Rewrite for IfC {
    type Target = Self;
    fn rewrite(self, _: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(self)
    }
}
