use crate::rewrite::{Rewrite, RewriteState};
use axcut::{
    syntax::statements::{Call, Invoke, Statement},
    traits::substitution::Subst,
};

use std::rc::Rc;

impl Rewrite for Invoke {
    type Target = Statement;
    fn rewrite(self, state: &mut RewriteState) -> Self::Target {
        let Some((clause, position)) = state.get_create_clause(&self.var, &self.tag) else {
            return self.into();
        };
        *state.new_changes = true;

        if matches!(
            *clause.body,
            Statement::Exit(_) | Statement::Call(_) | Statement::Invoke(_)
        ) {
            // for leaf statements, we do not lift, but just substitute into them
            let subst = clause
                .context
                .into_iter_vars()
                .zip(self.args.into_iter_vars())
                .collect::<Vec<_>>();
            Rc::unwrap_or_clone(clause.body.subst_sim(&subst))
        } else {
            let (label, arg_positions, mut free_args) =
                state.lift_create_clause(clause, position, &self.var);
            for position in arg_positions {
                free_args.push(self.args.bindings[position].clone());
            }

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
                args: free_args.into(),
            }
            .into()
        }
    }
}
