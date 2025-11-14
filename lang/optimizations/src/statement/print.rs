use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::statements::PrintI64;

impl Rewrite for PrintI64 {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(PrintI64 {
            newline: self.newline,
            var: self.var,
            next: self.next.rewrite(ctx)?,
            free_vars_next: self.free_vars_next,
        })
    }
}
