use crate::{
    errors::Error,
    rewrite::{Rewrite, RewriteState},
};
use axcut::syntax::statements::Call;

impl Rewrite for Call {
    type Target = Self;
    fn rewrite(self, _: &mut RewriteState) -> Result<Self::Target, Error> {
        Ok(self)
    }
}
