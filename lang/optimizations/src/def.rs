use crate::rewrite::{Rewrite, RewriteState};

pub fn rewrite_def(def_position: usize, state: &mut RewriteState) {
    // swap the body of the current Definition with a temporary placeholder
    let current_body = std::mem::take(&mut state.defs[def_position].body);
    state.current_label = state.defs[def_position].name.clone();
    state.current_used_vars = state.defs[def_position].used_vars.clone();
    state.let_bindings.clear();
    state.create_bindings.clear();

    // write the rewritten body back into place
    state.defs[def_position].body = current_body.rewrite(state)
}
