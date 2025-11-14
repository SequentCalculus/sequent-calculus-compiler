use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::statements::Let;

impl Rewrite for Let {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        ctx.add_binding(&self);
        Ok(self)
    }
}
