use super::{Error, Inline, InlineContext};
use axcut::syntax::Arguments;

impl Inline for Arguments {
    type Target = Arguments;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error> {
        Ok(Arguments {
            entries: self.entries,
        })
    }
}
