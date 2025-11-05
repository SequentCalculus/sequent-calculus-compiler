use super::{Inline, InlineContext, fresh_var};
use axcut::syntax::Def;

impl Inline for Def {
    type Target = Def;
    fn inline(self, ctx: &mut InlineContext) -> Self::Target {
        for binding in self.context.bindings {}
        Def {
            name: self.name,
            context: todo!(),
            body: self.body.inline(ctx),
            used_vars: self.used_vars,
        }
    }
}
