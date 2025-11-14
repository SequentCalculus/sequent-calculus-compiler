use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::statements::PrintI64;

impl Rewrite for PrintI64 {
    type Target = Self;
    fn rewrite(self, _: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(self)
    }
}
