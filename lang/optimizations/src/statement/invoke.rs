use crate::{Error, Rewrite, RewriteContext, free_bindings::FreeBindings};
use axcut::syntax::{
    ContextBinding,
    statements::{Call, Invoke, Statement},
};

impl Rewrite for Invoke {
    type Target = Statement;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        let create_clauses = match ctx.get_create(&self.var) {
            None => return Ok(self.into()),
            Some(cr) => cr,
        };
        ctx.new_changes = true;
        let clause_err = Error::create_clause(&create_clauses, &self.tag);
        let bind_rhs = create_clauses
            .into_iter()
            .find(|clause| clause.xtor == self.tag)
            .ok_or(clause_err)?;
        let lifted_name = ctx.create_lifted(&bind_rhs.xtor, &self.var);
        let mut args = self.args;
        let mut rhs_bindings: Vec<ContextBinding> = bind_rhs.free_bindings().into_iter().collect();
        rhs_bindings.sort();
        args.bindings.extend(rhs_bindings);

        if !ctx.already_lifted(&lifted_name) {
            ctx.lift_create_clause(bind_rhs, &self.var)?;
        }
        Ok(Call {
            label: lifted_name,
            args,
        }
        .into())
    }
}
