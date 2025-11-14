use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::statements::Substitute;

impl Rewrite for Substitute {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(Substitute {
            rearrange: self.rearrange,
            next: self.next.rewrite(ctx)?,
        })
    }
}
