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
            // we can always inline definitions only consisting of leaf statements, except for
            // `Call`s
            let subst = called_def
                .context
                .iter_vars_cloned()
                .zip(self.args.iter_vars_cloned())
                .collect::<Vec<_>>();

            called_def.body.clone().subst_sim(&subst)
        } else if matches!(&called_def.body, Statement::Call(_)) {
            // definitions only consisting of `Call`s could lead to non-termination if always
            // inlined recursively, because they could form cycles in the call graph; thus, we take
            // the body out temporarily when inlining recursively, so that we encounter a `Default`
            // statement when we have reached the last call before closing the cycle

            let subst = called_def
                .context
                .iter_vars_cloned()
                .zip(self.args.iter_vars_cloned())
                .collect::<Vec<_>>();

            // we always recursively visit the definitions only consisting of `Call`s, but we only
            // increment their mark on the first time for each call site
            if state.current_def_mark == Mark::Once {
                called_def_info.mark.increment();
            }

            let current_def_mark = state.current_def_mark;
            state.current_def_mark = called_def_info.mark;
            let called_def_body = std::mem::take(&mut state.defs[called_def_position].body);
            let inlined_body = called_def_body.clone().cleanup_inline_gather(state);
            state.defs[called_def_position].body = called_def_body;
            state.current_def_mark = current_def_mark;

            inlined_body.subst_sim(&subst)
        } else {
            // in all other cases we only gather the marking information ...
            called_def_info.mark.increment();
            if called_def_info.mark == Mark::Once
                && !matches!(&called_def.body, Statement::Default())
            {
                // ... and only visit the called `Def` if we have not done so yet
                let current_def_mark = state.current_def_mark;
                state.current_def_mark = called_def_info.mark;
                let called_def_body = std::mem::take(&mut state.defs[called_def_position].body);
                state.defs[called_def_position].body = called_def_body.cleanup_inline_gather(state);
                state.current_def_mark = current_def_mark;
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
