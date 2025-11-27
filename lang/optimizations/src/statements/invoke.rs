use crate::rewrite::{Rewrite, RewriteState};
use axcut::{
    syntax::statements::{Call, Invoke, Statement},
    traits::substitution::Subst,
};

use std::rc::Rc;

impl Rewrite for Invoke {
    type Target = Statement;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        let (clause, position) = match state.get_create_clause(&self.var, &self.tag) {
            None => return self.into(),
            Some(clause_and_position) => clause_and_position,
        };
        *state.new_changes = true;

        if matches!(
            *clause.body,
            Statement::Exit(_) | Statement::Call(_) | Statement::Invoke(_)
        ) {
            // for leaf statements, we do not lift, but just substitute into them
            if clause.context.bindings.len() != self.args.bindings.len() {
                panic!("Number of invoke arguments does not match number of clause arguments")
            }
            let subst = clause
                .context
                .bindings
                .into_iter()
                .map(|binding| binding.var)
                .zip(self.args.vars())
                .collect::<Vec<_>>();
            Rc::unwrap_or_clone(clause.body.subst_sim(&subst))
        } else {
            let mut args_clause = clause.context.clone();

            // lift the body of the clause to the top level
            let (label, free_vars) = state.lift_clause(clause, &self.var);

            // we have to rewrite the create whose clause we have lifted to avoid duplication
            args_clause.bindings.extend(free_vars.clone());
            let create = state.create_bindings.get_mut(&self.var).unwrap();
            create[position].body = Rc::new(
                Call {
                    label: label.clone(),
                    args: args_clause,
                }
                .into(),
            );

            self.args.bindings.extend(free_vars);
            Call {
                label,
                args: self.args,
            }
            .into()
        }
    }
}
