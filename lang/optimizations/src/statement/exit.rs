use crate::{Inline, InlineContext};
use axcut::syntax::statements::Exit;

impl Inline for Exit {
    type Target = Exit;
    fn inline(self, ctx: &mut InlineContext) -> Self::Target {
        Exit { var: self.var }
    }
}
