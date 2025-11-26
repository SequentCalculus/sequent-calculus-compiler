use crate::{Error, Rewrite, RewriteContext};
use axcut::syntax::{
    Def, Name, TypingContext, Var,
    statements::{Call, Clause, Invoke, Statement, Switch},
};
use std::{collections::HashSet, rc::Rc};

impl Rewrite for Call {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        let called_def = match ctx.get_def(&self.label) {
            None => return Ok(self),
            Some(df) => df,
        };
        match called_def.body {
            Statement::Switch(ref sw) => rewrite_call_switch(
                self,
                sw.clone(),
                ctx,
                &called_def.name,
                called_def.context,
                called_def.used_vars,
            ),
            Statement::Invoke(inv) => {
                rewrite_call_invoke(self, inv, ctx, &called_def.name, called_def.context)
            }
            _ => return Ok(self),
        }
    }
}

fn translate_name(
    name_in_def: &Var,
    caller_args: &TypingContext,
    def_args: &TypingContext,
    def_name: &Name,
) -> Result<Var, Error> {
    let def_ind = def_args
        .vars()
        .iter()
        .enumerate()
        .find(|(_, v)| *v == name_in_def)
        .ok_or(Error::VariableNotFound {
            var: name_in_def.clone(),
            context: def_args.clone(),
        })?
        .0;
    if caller_args.bindings.len() < def_ind {
        return Err(Error::CallArity {
            def: def_name.clone(),
            def_args: def_args.bindings.len(),
            called_args: caller_args.bindings.len(),
        });
    }
    Ok(caller_args.bindings[def_ind].var.clone())
}

fn rewrite_call_switch(
    call: Call,
    mut sw: Switch,
    ctx: &mut RewriteContext,
    def_name: &Name,
    def_args: TypingContext,
    def_used_vars: HashSet<Var>,
) -> Result<Call, Error> {
    let translated_name = translate_name(&sw.var, &call.args, &def_args, def_name)?;
    let (let_xtor, mut let_args) = match ctx.get_let(&translated_name) {
        None => return Ok(call),
        Some(lt) => lt,
    };
    let clause_ind = sw
        .clauses
        .iter()
        .position(|clause| clause.xtor == let_xtor)
        .ok_or(Error::XtorNotFound {
            xtor: let_xtor.clone(),
            clause_xtors: sw.clauses.iter().map(|cl| cl.xtor.clone()).collect(),
        })?;
    let xtor_clause = sw.clauses.remove(clause_ind);
    let lifted_name = ctx.switch_lifted(&def_name, &let_xtor, &sw.var);
    let extra_args = def_args
        .bindings
        .iter()
        .filter(|bnd| bnd.var != sw.var)
        .cloned()
        .collect::<Vec<_>>();
    let_args.bindings.extend(extra_args.clone());
    if !ctx.already_lifted(&lifted_name) {
        ctx.lift_switch_call(&def_name, &sw.var, &xtor_clause, extra_args);
    }
    let call_stmt = Call {
        label: lifted_name,
        args: let_args,
    };
    sw.clauses.push(Clause {
        xtor: let_xtor,
        context: xtor_clause.context.clone(),
        body: Rc::new(call_stmt.clone().into()),
    });

    let def_updated = Def {
        name: def_name.clone(),
        context: def_args,
        body: sw.into(),
        used_vars: def_used_vars,
    };
    ctx.add_def(def_updated);
    Ok(call_stmt)
}

fn rewrite_call_invoke(
    call: Call,
    inv: Invoke,
    ctx: &mut RewriteContext,
    def_name: &Name,
    def_args: TypingContext,
) -> Result<Call, Error> {
    let translated_name = translate_name(&inv.var, &call.args, &def_args, def_name)?;
    let mut clauses = match ctx.get_create(&translated_name) {
        None => return Ok(call),
        Some(clauses) => clauses,
    };
    let xtor_ind = clauses
        .iter()
        .position(|clause| clause.xtor == inv.tag)
        .ok_or(Error::XtorNotFound {
            xtor: inv.tag.clone(),
            clause_xtors: clauses.iter().map(|cl| cl.xtor.clone()).collect(),
        })?;
    let xtor_clause = clauses.remove(xtor_ind);
    let lifted_name = ctx.create_lifted(def_name, &xtor_clause.xtor, &translated_name);
    if !ctx.already_lifted(&lifted_name) {
        ctx.lift_create_call(def_name, &inv.var, xtor_clause);
    }
    Ok(Call {
        label: lifted_name,
        args: inv.args,
    })
}
