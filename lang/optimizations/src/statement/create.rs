use crate::{Inline, InlineContext};
use axcut::syntax::statements::Create;

impl Inline for Create {
    type Target = Create;
    fn inline(self, ctx: &mut InlineContext) -> Self::Target {
        Create {
            var: self.var,
            ty: self.ty,
            context: self.context.map(|ct| ct.inline(ctx)),
            clauses: self
                .clauses
                .into_iter()
                .map(|clause| clause.inline(ctx))
                .collect(),
            free_vars_clauses: self.free_vars_clauses,
            free_vars_next: self.free_vars_next,
            next: self.next.inline(ctx),
        }
    }
}
