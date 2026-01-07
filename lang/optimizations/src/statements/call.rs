use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState, Mark, Rename};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::{
    syntax::statements::{Call, Statement},
    traits::substitution::Subst,
};

use std::{collections::HashSet, rc::Rc};

impl Rewrite for Call {
    type Target = Statement;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        // we check whether the called function is a `Switch`, and if so, we temporarily remove the
        // `Switch` from its definition
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

impl CleanupInlineGather for Call {
    type Target = Statement;
    fn cleanup_inline_gather(self, state: &mut CleanupInlineState) -> Self::Target {
        let called_def_info = state
            .def_map
            .get_mut(&self.label)
            .unwrap_or_else(|| panic!("Called label {} must exist", self.label));
        let called_def_position = called_def_info.position;
        let called_def = &state.defs[called_def_position];
        if matches!(&called_def.body, Statement::Invoke(_) | Statement::Exit(_)) {
            // we can always inline leaf statements, except for `Call`s
            let subst = called_def
                .context
                .iter_vars_cloned()
                .zip(self.args.iter_vars_cloned())
                .collect::<Vec<_>>();

            called_def.body.clone().subst_sim(&subst)
        } else {
            let called_def_mark = &mut called_def_info.mark;
            let called_def_mark_old = *called_def_mark;
            called_def_mark.increment();
            if called_def_mark_old == Mark::None {
                // if we have not yet visited the called `Def`, we do so now
                let called_def_body = std::mem::take(&mut state.defs[called_def_position].body);
                state.defs[called_def_position].body = called_def_body.cleanup_inline_gather(state);
            }
            self.into()
        }
    }
}

impl CleanupInline for Call {
    type Target = Statement;
    fn cleanup_inline(self, state: &mut CleanupInlineState) -> Self::Target {
        let called_def_info = state
            .def_map
            .get(&self.label)
            .unwrap_or_else(|| panic!("Called label {} must exist", self.label));

        if called_def_info.mark == Mark::Once {
            let called_def_position = called_def_info.position;
            let mut called_def_body = std::mem::take(&mut state.defs[called_def_position].body);

            // we first recursively inline into the definition to inline
            let mut called_def_used_vars =
                std::mem::take(&mut state.defs[called_def_position].used_vars);
            std::mem::swap(&mut state.used_vars, &mut called_def_used_vars);
            called_def_body = called_def_body.cleanup_inline(state);
            std::mem::swap(&mut state.used_vars, &mut called_def_used_vars);

            let called_def = &state.defs[called_def_position];
            // to keep names after inlining unique and to avoid accidental capture, we have to
            // rename the binders that clash (the parameters are not renamed but later substituted);
            // we could avoid this by making all names unique across branches and definitions when
            // uniquifing
            let vars_to_rename: HashSet<_> = called_def_used_vars
                .intersection(&state.used_vars)
                .cloned()
                .collect();
            // the fresh names must occur neither in the current definition nor in the inlined one
            state.used_vars.extend(called_def_used_vars);
            if !vars_to_rename.is_empty() {
                called_def_body = called_def_body.rename(&vars_to_rename, &mut state.used_vars);
            }

            // now wo substitute the parameters
            let subst = called_def
                .context
                .iter_vars_cloned()
                .zip(self.args.iter_vars_cloned())
                .collect::<Vec<_>>();
            called_def_body = called_def_body.subst_sim(&subst);

            called_def_body
        } else {
            self.into()
        }
    }
}
