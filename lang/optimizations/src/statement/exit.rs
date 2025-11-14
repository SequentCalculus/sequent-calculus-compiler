use crate::{Error, GetUsedVars, Rewrite, RewriteContext};
use axcut::syntax::{Var, statements::Exit};
use std::collections::HashSet;

impl Rewrite for Exit {
    type Target = Self;
    fn rewrite(self, _: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(self)
    }
}

impl GetUsedVars for Exit {
    fn get_used_vars(&self) -> HashSet<Var> {
        HashSet::from([self.var.clone()])
    }
}
