use crate::{Error, Rewrite, RewriteContext, free_bindings::FreeBindings};
use axcut::syntax::statements::{Call, Clause, Create, Statement};
use std::rc::Rc;

impl Rewrite for Create {
    type Target = Statement;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        ctx.add_create(&self);
        let mut new_clauses = vec![];
        for clause in self.clauses {
            let lifted_name = ctx.create_lifted(&ctx.current_def, &clause.xtor, &self.var);
            let new_clause = if ctx.already_lifted(&lifted_name) {
                let new_def = ctx
                    .definitions
                    .iter()
                    .find(|def| def.name == lifted_name)
                    .ok_or(Error::DefinitionNotFound {
                        name: lifted_name.clone(),
                    })?;
                let mut new_args = clause.context.clone();
                let remaining_args = new_def
                    .context
                    .bindings
                    .clone()
                    .split_off(new_args.bindings.len());
                new_args.bindings.extend(remaining_args);
                ctx.new_changes = true;
                Clause {
                    xtor: clause.xtor,
                    context: clause.context,
                    body: Rc::new(
                        Call {
                            label: lifted_name,
                            args: new_args,
                        }
                        .into(),
                    ),
                }
            } else {
                clause.rewrite(ctx)?
            };
            new_clauses.push(new_clause);
        }
        let new_next = self.next.rewrite(ctx)?;
        let next_free = new_next.free_bindings();
        if next_free.iter().all(|bnd| bnd.var != self.var) {
            Ok(Rc::unwrap_or_clone(new_next))
        } else {
            Ok(Create {
                var: self.var,
                ty: self.ty,
                context: self.context,
                clauses: new_clauses,
                free_vars_clauses: None,
                next: new_next,
                free_vars_next: None,
            }
            .into())
        }
    }
}
