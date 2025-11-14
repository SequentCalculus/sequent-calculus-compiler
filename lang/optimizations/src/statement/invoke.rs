use crate::{Error, GetUsedVars, Rewrite, RewriteContext};
use axcut::syntax::{
    Var,
    statements::{Call, Invoke, Statement},
};
use std::collections::HashSet;

impl Rewrite for Invoke {
    type Target = Statement;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        let create_binding = match ctx.get_create(&self.var) {
            None => return Ok(self.into()),
            Some(cr) => cr,
        };
        let clause_err = Error::create_clause(&create_binding, &self.tag);
        let bind_rhs = create_binding
            .clauses
            .into_iter()
            .find(|clause| clause.xtor == self.tag)
            .ok_or(clause_err)?;
        let lifted_name = ctx.lifted_name(&bind_rhs.xtor);
        if !ctx.already_lifted(&lifted_name) {
            ctx.lift_clause(bind_rhs)?;
        }
        Ok(Call {
            label: lifted_name,
            args: self.args,
        }
        .into())
    }
}

impl GetUsedVars for Invoke {
    fn get_used_vars(&self) -> HashSet<Var> {
        let mut used = HashSet::from([self.var.clone()]);
        used.extend(self.args.entries.iter().cloned());
        used
    }
}
