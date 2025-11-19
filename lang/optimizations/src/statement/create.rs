use crate::{Error, Rewrite, RewriteContext, free_bindings::FreeBindings};
use axcut::syntax::statements::{Create, Statement};
use std::rc::Rc;

impl Rewrite for Create {
    type Target = Statement;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        ctx.add_create(&self);
        let mut new_clauses = vec![];
        for clause in self.clauses {
            let new_clause = clause.rewrite(ctx)?;
            new_clauses.push(new_clause);
        }
        let new_next = self.next.rewrite(ctx)?;
        if !new_next
            .free_bindings()
            .iter()
            .all(|bnd| bnd.var != self.var)
        {
            Ok(Rc::unwrap_or_clone(new_next))
        } else {
            Ok(Create {
                var: self.var,
                ty: self.ty,
                context: self.context,
                clauses: new_clauses,
                free_vars_clauses: None,
                next: new_next,
                free_vars_next: None,
            }
            .into())
        }
    }
}
