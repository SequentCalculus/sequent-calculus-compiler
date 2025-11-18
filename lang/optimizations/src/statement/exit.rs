use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::statements::Exit;

impl Rewrite for Exit {
    type Target = Self;
    fn rewrite(self, _: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(self)
    }
}
