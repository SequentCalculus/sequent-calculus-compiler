use crate::{Inline, InlineContext};
use axcut::syntax::statements::Let;

impl Inline for Let {
    type Target = Let;
    fn inline(self, ctx: &mut InlineContext) -> Self::Target {
        Let {
            var: self.var,
            ty: self.ty,
            tag: self.tag,
            args: self.args.inline(ctx),
            next: self.next.inline(ctx),
            free_vars_next: self.free_vars_next,
        }
    }
}
