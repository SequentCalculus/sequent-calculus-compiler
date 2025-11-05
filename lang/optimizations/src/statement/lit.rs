use crate::{Error, Inline, InlineContext};
use axcut::syntax::statements::Literal;

impl Inline for Literal {
    type Target = Literal;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error> {
        Ok(Literal {
            lit: self.lit,
            var: self.var,
            next: self.next.inline(ctx)?,
            free_vars_next: self.free_vars_next,
        })
    }
}
