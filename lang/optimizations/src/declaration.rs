use super::{Error, Inline, InlineContext};
use axcut::syntax::TypeDeclaration;

impl Inline for TypeDeclaration {
    type Target = TypeDeclaration;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error> {
        Ok(TypeDeclaration {
            name: self.name,
            xtors: self
                .xtors
                .into_iter()
                .map(|xtor| xtor.inline(ctx))
                .collect::<Result<Vec<_>, Error>>()?,
        })
    }
}
