use crate::{
    errors::Error,
    rewrite::{Rewrite, RewriteState},
};
use axcut::syntax::statements::Clause;

impl Rewrite for Clause {
    type Target = Self;
    fn rewrite(mut self, state: &mut RewriteState) -> Result<Self::Target, Error> {
        self.body = self.body.rewrite(state)?;
        Ok(self)
    }
}
