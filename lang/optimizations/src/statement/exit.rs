use crate::{Error, Inline, InlineContext};
use axcut::syntax::statements::Exit;

impl Inline for Exit {
    type Target = Exit;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error> {
        Ok(Exit { var: self.var })
    }
}
