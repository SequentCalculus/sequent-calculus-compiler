use crate::{
    errors::Error,
    rewrite::{Rewrite, RewriteState},
};
use axcut::syntax::statements::Exit;

impl Rewrite for Exit {
    type Target = Self;
    fn rewrite(self, _: &mut RewriteState) -> Result<Self::Target, Error> {
        Ok(self)
    }
}
