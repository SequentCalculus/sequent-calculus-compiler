use crate::{Inline, InlineContext};
use axcut::syntax::statements::PrintI64;

impl Inline for PrintI64 {
    type Target = PrintI64;
    fn inline(self, ctx: &mut InlineContext) -> Self::Target {
        PrintI64 {
            newline: self.newline,
            var: self.var,
            next: self.next.inline(ctx),
            free_vars_next: self.free_vars_next,
        }
    }
}
