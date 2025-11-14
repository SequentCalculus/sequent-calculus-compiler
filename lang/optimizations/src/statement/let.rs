use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::statements::Let;

impl Rewrite for Let {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        ctx.add_binding(&self);
        Ok(Let {
            var: self.var,
            ty: self.ty,
            tag: self.tag,
            args: self.args,
            next: self.next.rewrite(ctx)?,
            free_vars_next: self.free_vars_next,
        })
    }
}
