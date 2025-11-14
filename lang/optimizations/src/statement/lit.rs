use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::statements::Literal;

impl Rewrite for Literal {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(Literal {
            lit: self.lit,
            var: self.var,
            next: self.next.rewrite(ctx)?,
            free_vars_next: self.free_vars_next,
        })
    }
}
