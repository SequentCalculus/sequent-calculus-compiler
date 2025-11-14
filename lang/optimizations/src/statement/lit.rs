use crate::{Error, GetUsedVars, Rewrite, RewriteContext};
use axcut::{
    syntax::{Var, statements::Literal},
    traits::free_vars::FreeVars,
};
use std::collections::HashSet;

impl Rewrite for Literal {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        let mut free_next = HashSet::new();
        let new_next = self.next.rewrite(ctx)?.free_vars(&mut free_next);
        Ok(Literal {
            lit: self.lit,
            var: self.var,
            next: new_next,
            free_vars_next: Some(free_next),
        })
    }
}

impl GetUsedVars for Literal {
    fn get_used_vars(&self) -> HashSet<Var> {
        let mut used = HashSet::from([self.var.clone()]);
        used.extend(self.next.get_used_vars());
        used
    }
}
