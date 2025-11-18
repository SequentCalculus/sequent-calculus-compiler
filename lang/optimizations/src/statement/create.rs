use crate::{Error, Rewrite, RewriteContext};
use axcut::{
    syntax::statements::{Create, Statement},
    traits::free_vars::FreeVars,
};
use std::{collections::HashSet, rc::Rc};

impl Rewrite for Create {
    type Target = Statement;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        ctx.add_create(&self);
        let mut new_clauses = vec![];
        let mut free_clauses = HashSet::new();
        for clause in self.clauses {
            let new_clause = clause.rewrite(ctx)?.free_vars(&mut free_clauses);
            new_clauses.push(new_clause);
        }
        let mut free_next = HashSet::new();
        let new_next = self.next.rewrite(ctx)?.free_vars(&mut free_next);
        if !free_next.contains(&self.var) {
            Ok(Rc::unwrap_or_clone(new_next))
        } else {
            Ok(Create {
                var: self.var,
                ty: self.ty,
                context: self.context,
                clauses: new_clauses,
                free_vars_clauses: Some(free_clauses),
                next: new_next,
                free_vars_next: Some(free_next),
            }
            .into())
        }
    }
}
