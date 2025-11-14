use crate::{Error, GetUsedVars, Rewrite, RewriteContext};
use axcut::syntax::{Var, statements::Call};
use std::collections::HashSet;

impl Rewrite for Call {
    type Target = Self;
    fn rewrite(self, _: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(self)
    }
}

impl GetUsedVars for Call {
    fn get_used_vars(&self) -> HashSet<Var> {
        self.args.entries.iter().cloned().collect()
    }
}
