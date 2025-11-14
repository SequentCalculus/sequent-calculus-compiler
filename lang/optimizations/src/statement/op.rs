use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::statements::Op;

impl Rewrite for Op {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(Op {
            fst: self.fst,
            op: self.op,
            snd: self.snd,
            var: self.var,
            next: self.next.rewrite(ctx)?,
            free_vars_next: self.free_vars_next,
        })
    }
}
