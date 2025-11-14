use crate::{Error, GetUsedVars, Rewrite, RewriteContext};
use axcut::{
    syntax::{Var, statements::PrintI64},
    traits::free_vars::FreeVars,
};
use std::collections::HashSet;

impl Rewrite for PrintI64 {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        let mut free_next = HashSet::new();
        let new_next = self.next.rewrite(ctx)?.free_vars(&mut free_next);
        Ok(PrintI64 {
            newline: self.newline,
            var: self.var,
            next: new_next,
            free_vars_next: Some(free_next),
        })
    }
}

impl GetUsedVars for PrintI64 {
    fn get_used_vars(&self) -> HashSet<Var> {
        let mut used = HashSet::from([self.var.clone()]);
        used.extend(self.next.get_used_vars());
        used
    }
}
