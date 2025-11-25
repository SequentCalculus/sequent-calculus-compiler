use crate::{Error, Rewrite, RewriteContext, free_bindings::FreeBindings};
use axcut::syntax::{
    Def, TypingContext, Var,
    statements::{Call, Statement},
};

impl Rewrite for Call {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        let called_def = match ctx.get_def(&self.label) {
            None => return Ok(self),
            Some(df) => df,
        };
        match called_def.body {
            Statement::Switch(ref sw) => {
                let translated_name = translate_name(&sw.var, &self.args, &called_def)?;
                let (let_xtor, mut let_args) = match ctx.get_let(&translated_name) {
                    None => return Ok(self),
                    Some(lt) => lt,
                };
                let xtor_clause = sw.clauses.iter().find(|cl| cl.xtor == let_xtor).ok_or(
                    Error::XtorNotFound {
                        xtor: let_xtor.clone(),
                        clause_xtors: sw.clauses.iter().map(|cl| cl.xtor.clone()).collect(),
                    },
                )?;
                let_args.bindings.extend(xtor_clause.body.free_bindings());
                let lifted_name = ctx.switch_lifted(&called_def.name, &let_xtor, &sw.var);
                if !ctx.already_lifted(&lifted_name) {
                    ctx.lift_switch_clause(&called_def.name, &sw.var, xtor_clause);
                }
                Ok(Call {
                    label: lifted_name,
                    args: let_args,
                })
            }
            Statement::Invoke(cr)
                if self.args.vars().contains(&translate_name(
                    &cr.var,
                    &self.args,
                    &called_def,
                )?) =>
            {
                todo!()
            }
            _ => return Ok(self),
        }
    }
}

fn translate_name(
    name_in_def: &Var,
    caller_args: &TypingContext,
    called_def: &Def,
) -> Result<Var, Error> {
    let def_ind = called_def
        .context
        .vars()
        .iter()
        .enumerate()
        .find(|(_, v)| *v == name_in_def)
        .ok_or(Error::VariableNotFound {
            var: name_in_def.clone(),
            context: called_def.context.clone(),
        })?
        .0;
    if caller_args.bindings.len() < def_ind {
        return Err(Error::CallArity {
            def: called_def.name.clone(),
            def_args: called_def.context.bindings.len(),
            called_args: caller_args.bindings.len(),
        });
    }
    Ok(caller_args.bindings[def_ind].var.clone())
}
