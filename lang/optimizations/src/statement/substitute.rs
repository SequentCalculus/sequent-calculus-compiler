use crate::{Error, Inline, InlineContext};
use axcut::syntax::statements::Substitute;

impl Inline for Substitute {
    type Target = Substitute;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error> {
        Ok(Substitute {
            rearrange: self.rearrange,
            next: self.next.inline(ctx)?,
        })
    }
}
