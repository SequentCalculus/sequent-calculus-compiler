use super::{Error, Inline, InlineContext};
use axcut::syntax::{ContextBinding, TypingContext};

impl Inline for TypingContext {
    type Target = TypingContext;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error> {
        Ok(TypingContext {
            bindings: self
                .bindings
                .into_iter()
                .map(|bind| bind.inline(ctx))
                .collect::<Result<Vec<_>, Error>>()?,
        })
    }
}

impl Inline for ContextBinding {
    type Target = ContextBinding;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error> {
        Ok(ContextBinding {
            var: self.var,
            chi: self.chi,
            ty: self.ty,
        })
    }
}
