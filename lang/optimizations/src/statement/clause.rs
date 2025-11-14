use crate::{Error, GetUsedVars, Rewrite, RewriteContext};
use axcut::syntax::{Var, statements::Clause};
use std::collections::HashSet;

impl Rewrite for Clause {
    type Target = Clause;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(Clause {
            xtor: self.xtor,
            context: self.context,
            body: self.body.rewrite(ctx)?,
        })
    }
}

impl GetUsedVars for Clause {
    fn get_used_vars(&self) -> HashSet<Var> {
        &self.body.get_used_vars()
            | &self
                .context
                .bindings
                .iter()
                .map(|bind| &bind.var)
                .cloned()
                .collect()
    }
}
