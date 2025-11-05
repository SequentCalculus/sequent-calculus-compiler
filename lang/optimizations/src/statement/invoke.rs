use crate::{Error, Inline, InlineContext};
use axcut::syntax::statements::Invoke;

impl Inline for Invoke {
    type Target = Invoke;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error> {
        Ok(Invoke {
            var: self.var,
            tag: self.tag,
            ty: self.ty,
            args: self.args.inline(ctx)?,
        })
    }
}
