use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::{
    Def, TypingContext, Var,
    statements::{Call, Clause, Invoke, Statement, Switch},
};
use std::rc::Rc;

impl Rewrite for Call {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        let called_def = match ctx.get_def(&self.label) {
            None => return Ok(self),
            Some(df) => df,
        };
        match called_def.body {
            Statement::Switch(ref sw) => rewrite_call_switch(self, sw.clone(), called_def, ctx),
            Statement::Invoke(inv) => rewrite_call_invoke(inv),
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

fn rewrite_call_switch(
    call: Call,
    sw: Switch,
    called_def: Def,
    ctx: &mut RewriteContext,
) -> Result<Call, Error> {
    let translated_name = translate_name(&sw.var, &call.args, &called_def)?;
    let (let_xtor, mut let_args) = match ctx.get_let(&translated_name) {
        None => return Ok(call),
        Some(lt) => lt,
    };
    let xtor_clause =
        sw.clauses
            .iter()
            .find(|cl| cl.xtor == let_xtor)
            .ok_or(Error::XtorNotFound {
                xtor: let_xtor.clone(),
                clause_xtors: sw.clauses.iter().map(|cl| cl.xtor.clone()).collect(),
            })?;
    let lifted_name = ctx.switch_lifted(&called_def.name, &let_xtor, &sw.var);
    let extra_args = called_def
        .context
        .bindings
        .iter()
        .filter(|bnd| bnd.var != sw.var)
        .cloned()
        .collect::<Vec<_>>();
    let_args.bindings.extend(extra_args.clone());
    if !ctx.already_lifted(&lifted_name) {
        ctx.lift_switch_clause(&called_def.name, &sw.var, xtor_clause, extra_args);
    }
    let call_stmt = Call {
        label: lifted_name,
        args: let_args,
    };
    let mut new_clauses = sw.clauses.clone();
    new_clauses.retain(|clause| clause.xtor != let_xtor);
    new_clauses.push(Clause {
        xtor: let_xtor,
        context: xtor_clause.context.clone(),
        body: Rc::new(call_stmt.clone().into()),
    });

    let def_updated = Def {
        name: called_def.name,
        context: called_def.context,
        body: Switch {
            var: sw.var,
            ty: sw.ty,
            clauses: new_clauses,
            free_vars_clauses: None,
        }
        .into(),
        used_vars: called_def.used_vars,
    };
    ctx.add_def(def_updated);
    Ok(call_stmt)
}

fn rewrite_call_invoke(_: Invoke) -> Result<Call, Error> {
    todo!()
}
