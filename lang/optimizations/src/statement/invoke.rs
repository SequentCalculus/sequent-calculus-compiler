use crate::{Error, Rewrite, RewriteContext, free_bindings::FreeBindings};
use axcut::syntax::{
    ContextBinding,
    statements::{Call, Invoke, Statement},
};

impl Rewrite for Invoke {
    type Target = Statement;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        let create_binding = match ctx.get_create(&self.var) {
            None => return Ok(self.into()),
            Some(cr) => cr,
        };
        ctx.new_changes = true;
        let clause_err = Error::create_clause(&create_binding, &self.tag);
        let bind_rhs = create_binding
            .clauses
            .into_iter()
            .find(|clause| clause.xtor == self.tag)
            .ok_or(clause_err)?;
        let lifted_name = ctx.lifted_name(&bind_rhs.xtor, &self.var);
        let mut context = self.context;
        let mut rhs_bindings: Vec<ContextBinding> = bind_rhs.free_bindings().into_iter().collect();
        rhs_bindings.sort();
        context.bindings.extend(rhs_bindings);

        if !ctx.already_lifted(&lifted_name) {
            ctx.lift_clause(bind_rhs, &self.var)?;
        }
        Ok(Call {
            label: lifted_name,
            context,
        }
        .into())
    }
}
