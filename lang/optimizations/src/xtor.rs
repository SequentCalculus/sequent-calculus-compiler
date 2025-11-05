use super::{Inline, InlineContext};
use axcut::syntax::XtorSig;

impl Inline for XtorSig {
    type Target = XtorSig;
    fn inline(self, ctx: &mut InlineContext) -> Self::Target {
        XtorSig {
            name: self.name,
            args: self.args.inline(ctx),
        }
    }
}
