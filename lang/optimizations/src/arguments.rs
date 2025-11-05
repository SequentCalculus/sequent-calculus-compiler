use super::{Inline, InlineContext};
use axcut::syntax::Arguments;

impl Inline for Arguments {
    type Target = Arguments;
    fn inline(self, ctx: &mut InlineContext) -> Self::Target {
        Arguments {
            entries: self.entries,
        }
    }
}
