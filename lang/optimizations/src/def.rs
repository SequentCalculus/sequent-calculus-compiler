use crate::rewrite::{Rewrite, RewriteState, SwitchInfo};
use axcut::syntax::statements::Statement;

pub fn rewrite_def(def_position: usize, state: &mut RewriteState) {
    // swap the body of the current `Def`inition with a temporary placeholder
    let current_body = std::mem::take(&mut state.defs[def_position].body);
    let rewritten_body = if matches!(current_body, Statement::Switch(_)) {
        // if the body is a `Switch`, we lift all its non-leaf `Clause`s to facilitate rewriting of
        // direct recursive calls; a later cleanup inlining pass will undo unnecessary lifts
        let Statement::Switch(switch) = current_body else {
            unreachable!("we already know that the body is a Switch")
        };
        let number_of_clauses = switch.clauses.len();
        let mut switch_info = SwitchInfo {
            switch,
            clause_position: 0,
            called_def_position: def_position,
            let_args: Vec::default().into(),
        };
        let label = state.defs[def_position].name.clone();

        for clause_position in 0..number_of_clauses {
            // only rewrite non-leaf statements
            if !matches!(
                &*switch_info.switch.clauses[clause_position].body,
                Statement::Call(_) | Statement::Invoke(_) | Statement::Exit(_)
            ) {
                switch_info.clause_position = clause_position;
                state.lift_switch_clause(&mut switch_info, &label);
            }
        }

        switch_info.switch.into()
    } else {
        state.current_label = state.defs[def_position].name.clone();
        state.current_used_vars = state.defs[def_position].used_vars.clone();
        state.let_bindings.clear();
        state.create_bindings.clear();

        current_body.rewrite(state)
    };

    // write the rewritten body back into place
    state.defs[def_position].body = rewritten_body;
}
