use crate::{
    errors::Error,
    rewrite::{Rewrite, RewriteState},
};
use axcut::syntax::statements::IfC;

impl Rewrite for IfC {
    type Target = Self;
    fn rewrite(mut self, state: &mut RewriteState) -> Result<Self::Target, Error> {
        self.thenc = self.thenc.rewrite(state)?;
        self.elsec = self.elsec.rewrite(state)?;
        Ok(self)
    }
}
