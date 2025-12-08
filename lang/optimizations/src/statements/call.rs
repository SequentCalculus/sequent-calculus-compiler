use crate::rewrite::{Rewrite, RewriteState};
use axcut::{
    syntax::{
        Def, Name, TypingContext, Var,
        names::fresh_name,
        statements::{Call, Clause, Statement},
    },
    traits::{substitution::Subst, typed_free_vars::TypedFreeVars},
};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    rc::Rc,
};

struct SwitchContext<'a> {
    switch_clause: &'a mut Clause,
    switch_var: &'a Var,
    let_args: TypingContext,
    def_context: TypingContext,
    def_used_vars: HashSet<Var>,
    switch_arg_ind: usize,
}

fn get_switch<'a>(
    lifted_statements: &'a mut [Def],
    let_bindings: &'a HashMap<Var, (Name, TypingContext)>,
    called_label: &Name,
    called_args: &TypingContext,
) -> Option<SwitchContext<'a>> {
    let called_def = lifted_statements
        .iter_mut()
        .find(|df| df.name == *called_label)?;
    let switch = match &mut called_def.body {
        Statement::Switch(sw) => sw,
        _ => {
            return None;
        }
    };
    let switch_arg_ind = called_def
        .context
        .bindings
        .iter()
        .position(|bind| bind.var == switch.var)
        .expect("Could not find switch variable");

    let (let_xtor, let_args) = let_bindings.get(&called_args.bindings[switch_arg_ind].var)?;

    let switch_clause_ind = switch
        .clauses
        .iter()
        .position(|clause| clause.xtor == *let_xtor)
        .expect("Could not find clause for xtor");
    let switch_clause = &mut switch.clauses[switch_clause_ind];

    Some(SwitchContext {
        switch_clause,
        switch_var: &switch.var,
        let_args: let_args.clone(),
        def_context: called_def.context.clone(),
        def_used_vars: called_def.used_vars.clone(),
        switch_arg_ind,
    })
}

impl Rewrite for Call {
    type Target = Statement;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        let ctx = match get_switch(
            &mut state.lifted_statements,
            &state.let_bindings,
            &self.label,
            &self.args,
        ) {
            None => return self.into(),
            Some(def) => def,
        };
        state.new_changes = true;

        if matches!(
            &*ctx.switch_clause.body,
            Statement::Call(_) | Statement::Invoke(_) | Statement::Exit(_)
        ) {
            return inline_leaf(
                ctx.switch_clause,
                &ctx.def_context.vars(),
                &self.args.vars(),
                &ctx.let_args.vars(),
            );
        }

        self.args.bindings.remove(ctx.switch_arg_ind);
        let old_args = self.args.bindings;
        self.args = ctx.let_args;
        self.args.bindings.extend(old_args);

        let lifted_name = fresh_name(
            &mut state.used_labels,
            &("lift_".to_string()
                + &self.label
                + "_"
                + ctx.switch_var
                + "_"
                + &ctx.switch_clause.xtor),
        );
        state.used_labels.insert(lifted_name.clone());

        let new_def = lifted_def(
            ctx.def_used_vars,
            ctx.switch_clause,
            ctx.def_context,
            ctx.switch_arg_ind,
            &lifted_name,
        );
        state.add_def(new_def);
        Call {
            label: lifted_name,
            args: self.args,
        }
        .into()
    }
}

fn inline_leaf(
    switch_clause: &Clause,
    def_args: &[Var],
    call_args: &[Var],
    let_args: &[Var],
) -> Statement {
    let mut subst = vec![];
    let mut free = BTreeSet::new();
    switch_clause.typed_free_vars(&mut free);
    for binding in free {
        let call_pos = def_args
            .iter()
            .position(|bind| *bind == binding.var)
            .expect("Could not find variable in definition");
        subst.push((def_args[call_pos].clone(), call_args[call_pos].clone()));
    }
    for (ind, bind) in switch_clause.context.bindings.iter().enumerate() {
        subst.push((bind.var.clone(), let_args[ind].clone()));
    }

    Rc::unwrap_or_clone(switch_clause.body.clone().subst_sim(&subst))
}

fn lifted_def(
    used_vars: HashSet<Var>,
    switch_clause: &mut Clause,
    mut call_args: TypingContext,
    switch_var_ind: usize,
    lifted_name: &Name,
) -> Def {
    let mut new_context = switch_clause.context.clone();
    call_args.bindings.remove(switch_var_ind);
    new_context.bindings.extend(call_args.bindings);

    let old_body = std::mem::replace(
        &mut switch_clause.body,
        Rc::new(
            Call {
                label: lifted_name.clone(),
                args: new_context.clone(),
            }
            .into(),
        ),
    );
    Def {
        name: lifted_name.clone(),
        context: new_context,
        body: Rc::unwrap_or_clone(old_body),
        used_vars,
    }
}
