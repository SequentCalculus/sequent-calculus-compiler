use crate::{Error, Rewrite, RewriteContext};
use axcut::{syntax::statements::Literal, traits::free_vars::FreeVars};
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
