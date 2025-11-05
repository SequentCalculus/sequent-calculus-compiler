use super::{Error, Inline, InlineContext};
use axcut::syntax::XtorSig;

impl Inline for XtorSig {
    type Target = XtorSig;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error> {
        Ok(XtorSig {
            name: self.name,
            args: self.args.inline(ctx)?,
        })
    }
}
