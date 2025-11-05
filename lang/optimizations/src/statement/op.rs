use crate::{Inline, InlineContext};
use axcut::syntax::statements::Op;

impl Inline for Op {
    type Target = Op;
    fn inline(self, ctx: &mut InlineContext) -> Self::Target {
        Op {
            fst: self.fst,
            snd: self.snd,
            op: self.op,
            var: self.var,
            next: self.next.inline(ctx),
            free_vars_next: self.free_vars_next,
        }
    }
}
