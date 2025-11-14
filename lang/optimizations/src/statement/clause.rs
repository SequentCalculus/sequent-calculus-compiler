use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::statements::Clause;

impl Rewrite for Clause {
    type Target = Clause;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(Clause {
            xtor: self.xtor,
            context: self.context,
            body: self.body.rewrite(ctx)?,
        })
    }
}
