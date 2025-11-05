use crate::{Inline, InlineContext};
use axcut::syntax::statements::Switch;

impl Inline for Switch {
    type Target = Switch;
    fn inline(self, ctx: &mut InlineContext) -> Self::Target {
        Switch {
            var: self.var,
            ty: self.ty,
            clauses: self
                .clauses
                .into_iter()
                .map(|clause| clause.inline(ctx))
                .collect(),
            free_vars_clauses: self.free_vars_clauses,
        }
    }
}
