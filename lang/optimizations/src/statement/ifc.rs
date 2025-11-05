use crate::{Inline, InlineContext};
use axcut::syntax::statements::IfC;

impl Inline for IfC {
    type Target = IfC;
    fn inline(self, ctx: &mut InlineContext) -> Self::Target {
        IfC {
            sort: self.sort,
            fst: self.fst,
            snd: self.snd,
            thenc: self.thenc.inline(ctx),
            elsec: self.elsec.inline(ctx),
        }
    }
}
