use crate::{Error, GetUsedVars, Rewrite, RewriteContext};
use axcut::syntax::{Var, statements::Substitute};
use std::collections::HashSet;

impl Rewrite for Substitute {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(Substitute {
            rearrange: self.rearrange,
            next: self.next.rewrite(ctx)?,
        })
    }
}

impl GetUsedVars for Substitute {
    fn get_used_vars(&self) -> HashSet<Var> {
        let mut used = self.next.get_used_vars();
        for (fst, snd) in self.rearrange.iter() {
            used.insert(fst.clone());
            used.insert(snd.clone());
        }
        used
    }
}
