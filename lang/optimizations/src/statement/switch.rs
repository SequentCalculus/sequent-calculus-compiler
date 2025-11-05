use crate::{Error, Inline, InlineContext};
use axcut::syntax::statements::Switch;

impl Inline for Switch {
    type Target = Switch;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error> {
        Ok(Switch {
            var: self.var,
            ty: self.ty,
            clauses: self
                .clauses
                .into_iter()
                .map(|clause| clause.inline(ctx))
                .collect::<Result<Vec<_>, Error>>()?,
            free_vars_clauses: self.free_vars_clauses,
        })
    }
}
