use super::{Inline, InlineContext};
use axcut::syntax::{ContextBinding, TypingContext};

impl Inline for TypingContext {
    type Target = TypingContext;
    fn inline(self, ctx: &mut InlineContext) -> Self::Target {
        TypingContext {
            bindings: self
                .bindings
                .into_iter()
                .map(|bind| bind.inline(ctx))
                .collect(),
        }
    }
}

impl Inline for ContextBinding {
    type Target = ContextBinding;
    fn inline(self, ctx: &mut InlineContext) -> Self::Target {
        ContextBinding {
            var: self.var,
            chi: self.chi,
            ty: self.ty,
        }
    }
}
