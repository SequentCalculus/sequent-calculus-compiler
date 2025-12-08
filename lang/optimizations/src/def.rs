use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::{Def, Name};

pub fn rewrite_def(name: &Name, state: &mut RewriteState) {
    let def_ind = state
        .lifted_statements
        .iter()
        .position(|def| def.name == *name)
        .expect("Could not find definition");
    let mut current_def = state.lifted_statements.remove(def_ind);
    state.set_current_def(name, current_def.used_vars.clone());
    current_def.body = current_def.body.rewrite(state);
    state.lifted_statements.insert(def_ind, current_def)
}
