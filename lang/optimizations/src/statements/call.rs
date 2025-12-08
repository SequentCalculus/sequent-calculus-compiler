use crate::rewrite::{Rewrite, RewriteState};
use axcut::{
    syntax::{
        TypingContext, Var,
        statements::{Call, Clause, Statement},
    },
    traits::{substitution::Subst, typed_free_vars::TypedFreeVars},
};
use std::{collections::BTreeSet, rc::Rc};

impl Rewrite for Call {
    type Target = Statement;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        let called_ind = match state
            .lifted_statements
            .iter()
            .position(|df| df.name == self.label)
        {
            None => return self.into(),
            Some(ind) => ind,
        };

        let mut called_def = state.lifted_statements.remove(called_ind);
        let switch = match &mut called_def.body {
            Statement::Switch(sw) => sw,
            _ => {
                state.add_def(called_def);
                return self.into();
            }
        };

        let switch_arg_ind = called_def
            .context
            .bindings
            .iter()
            .position(|bind| bind.var == switch.var)
            .expect("Could not find switch variable");

        let (let_xtor, mut let_args) = match state.get_let(&self.args.bindings[switch_arg_ind].var)
        {
            None => {
                state.add_def(called_def);
                return self.into();
            }
            Some(lt) => lt,
        };

        let switch_clause_ind = switch
            .clauses
            .iter()
            .position(|clause| clause.xtor == let_xtor)
            .expect("Could not find clause for xtor");
        let switch_clause = &mut switch.clauses[switch_clause_ind];
        state.new_changes = true;
        let (lifted_name, lifted_args) = match &*switch_clause.body {
            Statement::Call(_) | Statement::Invoke(_) | Statement::Exit(_) => {
                let return_stmt = inline_leaf(
                    &switch_clause,
                    &called_def.context.vars(),
                    &self.args.vars(),
                    &let_args.vars(),
                );

                state.add_def(called_def);
                return return_stmt;
            }
            _ => {
                self.args.bindings.remove(switch_arg_ind);
                let_args.bindings.extend(self.args.bindings);
                self.args = let_args;

                state.lift_switch_call(
                    &called_def.name,
                    &switch.var,
                    &called_def.context,
                    &called_def.used_vars,
                    &switch_clause,
                )
            }
        };

        switch_clause.body = Rc::new(
            Call {
                label: lifted_name.clone(),
                args: TypingContext {
                    bindings: lifted_args,
                },
            }
            .into(),
        );
        state.add_def(called_def);
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

    return Rc::unwrap_or_clone(switch_clause.body.clone().subst_sim(&subst));
}
