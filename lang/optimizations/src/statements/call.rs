use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::{
    Def, Name, TypingContext, Var,
    statements::{Call, Clause, Invoke, Statement, Switch},
};
use std::{collections::HashSet, rc::Rc};

impl Rewrite for Call {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteState) -> Self::Target {
        let called_def = match ctx
            .lifted_statements
            .iter()
            .find(|df| df.name == self.label)
        {
            None => return self,
            Some(df) => df.clone(),
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
            Statement::Invoke(inv) => rewrite_call_invoke(self, inv, ctx, called_def.context),
            _ => self,
        }
    }
}

fn translate_name(name_in_def: &Var, caller_args: &TypingContext, def_args: &TypingContext) -> Var {
    let def_ind = def_args
        .vars()
        .iter()
        .enumerate()
        .find(|(_, v)| *v == name_in_def)
        .expect("Variable not found in definition arguments")
        .0;
    if caller_args.bindings.len() < def_ind {
        panic!("Number of definition arguments and call arguments do not match")
    }
    caller_args.bindings[def_ind].var.clone()
}

fn rewrite_call_switch(
    call: Call,
    mut sw: Switch,
    ctx: &mut RewriteState,
    def_name: &Name,
    def_args: TypingContext,
    def_used_vars: HashSet<Var>,
) -> Call {
    let translated_name = translate_name(&sw.var, &call.args, &def_args);
    let (let_xtor, mut let_args) = match ctx.get_let(&translated_name) {
        None => return call,
        Some(lt) => lt,
    };
    let clause_ind = sw
        .clauses
        .iter()
        .position(|clause| clause.xtor == let_xtor)
        .expect("No matching Clause for Xtor");
    let xtor_clause = sw.clauses.remove(clause_ind);
    //TODO: replace `switch_lifted`with the correct name, making sure a statement is only lifted
    //once
    //let lifted_name = ctx.switch_lifted(def_name, &let_xtor, &sw.var);
    let lifted_name = String::new();
    let extra_args = def_args
        .bindings
        .iter()
        .filter(|bnd| bnd.var != sw.var)
        .cloned()
        .collect::<Vec<_>>();
    let_args.bindings.extend(extra_args.clone());
    /*if !ctx.already_lifted(&lifted_name) {
        ctx.lift_switch_call(def_name, &sw.var, &xtor_clause, extra_args);
    }*/
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
    ctx.new_changes = true;
    call_stmt
}

fn rewrite_call_invoke(
    call: Call,
    inv: Invoke,
    ctx: &mut RewriteState,
    def_args: TypingContext,
) -> Call {
    let translated_name = translate_name(&inv.var, &call.args, &def_args);
    let mut clauses = match ctx.create_bindings.get(&translated_name) {
        None => return call,
        Some(clauses) => clauses.clone(),
    };
    let xtor_ind = clauses
        .iter()
        .position(|clause| clause.xtor == inv.tag)
        .expect("No Clause for Xtor in Create");
    let xtor_clause = clauses.remove(xtor_ind);
    // TODO: replace `create_lifted` with correct name making sure a clause is not lifted twice
    //let lifted_name = ctx.create_lifted(def_name, &xtor_clause.xtor, &translated_name);
    let lifted_name = String::new();
    /*if !ctx.already_lifted(&lifted_name) {
        ctx.lift_create_call(def_name, &inv.var, xtor_clause);
    }*/
    ctx.new_changes = true;
    Call {
        label: lifted_name,
        args: call.args,
    }
}
