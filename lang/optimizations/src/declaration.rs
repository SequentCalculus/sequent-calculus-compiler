use super::{Inline, InlineContext};
use axcut::syntax::TypeDeclaration;

impl Inline for TypeDeclaration {
    type Target = TypeDeclaration;
    fn inline(self, ctx: &mut InlineContext) -> Self::Target {
        TypeDeclaration {
            name: self.name,
            xtors: self
                .xtors
                .into_iter()
                .map(|xtor| xtor.inline(ctx))
                .collect(),
        }
    }
}
