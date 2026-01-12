use crate::rewrite::{Rewrite, RewriteState};
use axcut::{
    syntax::statements::{Call, Statement},
    traits::substitution::Subst,
};
use std::rc::Rc;

impl Rewrite for Call {
    type Target = Statement;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        // here we temporarily remove the Switch from its definition
        let Some(mut switch_info) = state.get_switch_info(&self.label, &self.args) else {
            return self.into();
        };
        state.new_changes = true;

        let clause = &switch_info.switch.clauses[switch_info.clause_position];
        let result = if matches!(
            &*clause.body,
            Statement::Call(_) | Statement::Invoke(_) | Statement::Exit(_)
        ) {
            // for leaf statements, we do not lift, but just substitute into them
            let subst = clause
                .context
                .iter_vars_cloned()
                .zip(switch_info.let_args.iter_vars_cloned())
                .chain(
                    state.defs[switch_info.called_def_position]
                        .context
                        .iter_vars_cloned()
                        .zip(self.args.iter_vars_cloned()),
                )
                .collect::<Vec<_>>();

            Rc::unwrap_or_clone(clause.body.clone().subst_sim(&subst))
        } else {
            let (label, arg_positions) = state.lift_switch_clause(&mut switch_info, &self.label);
            self.args.bindings.extend(switch_info.let_args.bindings);
            let mut args = Vec::with_capacity(arg_positions.len());
            for position in arg_positions {
                args.push(self.args.bindings[position].clone());
            }

            Call {
                label,
                args: args.into(),
            }
            .into()
        };

        // now we have to put the (potentially lifted) Switch back into place
        state.defs[switch_info.called_def_position].body = switch_info.switch.into();
        result
    }
}
