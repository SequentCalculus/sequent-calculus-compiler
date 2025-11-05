use crate::{Error, Inline, InlineContext};
use axcut::syntax::statements::Call;

impl Inline for Call {
    type Target = Call;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error> {
        Ok(Call {
            label: self.label,
            args: self.args.inline(ctx)?,
        })
    }
}
