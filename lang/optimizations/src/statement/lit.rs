use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::statements::Literal;

impl Rewrite for Literal {
    type Target = Self;
    fn rewrite(self, _: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(self)
    }
}
