use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::statements::{Clause, Create};

impl Rewrite for Create {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        let new_clauses = self
            .clauses
            .into_iter()
            .map(|cl| cl.rewrite(ctx))
            .collect::<Result<Vec<Clause>, Error>>()?;
        let new_next = self.next.rewrite(ctx)?;
        Ok(Create {
            var: self.var,
            ty: self.ty,
            context: self.context,
            clauses: new_clauses,
            free_vars_clauses: self.free_vars_clauses,
            next: new_next,
            free_vars_next: self.free_vars_next,
        })
    }
}
