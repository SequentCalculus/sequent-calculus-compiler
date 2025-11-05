use super::{Inline, InlineContext};
use axcut::syntax::statements::Clause;

impl Inline for Clause {
    type Target = Clause;
    fn inline(self, ctx: &mut InlineContext) -> Self::Target {
        Clause {
            xtor: self.xtor,
            context: self.context.inline(ctx),
            body: self.body.inline(ctx),
        }
    }
}
