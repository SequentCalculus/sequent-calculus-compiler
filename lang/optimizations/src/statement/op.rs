use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::statements::Op;

impl Rewrite for Op {
    type Target = Self;
    fn rewrite(self, _: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(self)
    }
}
