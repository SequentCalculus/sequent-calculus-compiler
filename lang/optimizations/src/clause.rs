use super::{Error, Inline, InlineContext};
use axcut::syntax::statements::Clause;

impl Inline for Clause {
    type Target = Clause;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error> {
        Ok(Clause {
            xtor: self.xtor,
            context: self.context.inline(ctx)?,
            body: self.body.inline(ctx)?,
        })
    }
}
