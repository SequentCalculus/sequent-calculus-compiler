use crate::{Error, Inline, InlineContext};
use axcut::syntax::statements::Create;

impl Inline for Create {
    type Target = Create;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error> {
        Ok(Create {
            var: self.var,
            ty: self.ty,
            context: self.context.map(|ct| ct.inline(ctx)).transpose()?,
            clauses: self
                .clauses
                .into_iter()
                .map(|clause| clause.inline(ctx))
                .collect::<Result<Vec<_>, Error>>()?,
            free_vars_clauses: self.free_vars_clauses,
            free_vars_next: self.free_vars_next,
            next: self.next.inline(ctx)?,
        })
    }
}
