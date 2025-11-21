use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::statements::IfC;

impl Rewrite for IfC {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(IfC {
            sort: self.sort,
            fst: self.fst,
            snd: self.snd,
            thenc: self.thenc.rewrite(ctx)?,
            elsec: self.elsec.rewrite(ctx)?,
        })
    }
}
