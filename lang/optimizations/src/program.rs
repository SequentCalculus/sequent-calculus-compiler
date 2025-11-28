use crate::{def::rewrite_def, rewrite::RewriteState};
use axcut::syntax::Prog;
use std::{
    collections::{HashMap, HashSet},
    mem::take,
};

pub const MAX_RUNS: u64 = 10;

pub fn rewrite_prog(mut program: Prog) -> Prog {
    // we thread the set of labels of top-level functions through the translation, because we need
    // to generate fresh labels when we lift statements
    let used_labels = program.defs.iter().map(|def| def.name.clone()).collect();
    let defs = take(&mut program.defs);
    let mut state = RewriteState {
        used_labels: used_labels,
        lifted_statements: defs,
        current_label: "main".to_owned(),
        current_used_vars: HashSet::new(),
        let_bindings: HashMap::new(),
        create_bindings: HashMap::new(),
        new_changes: true,
    };

    let mut performed_runs = 0;
    while state.new_changes && performed_runs < MAX_RUNS {
        state.new_changes = false;
        performed_runs += 1;
        let current_labels = state.used_labels.iter().cloned().collect::<Vec<_>>();
        for def_name in current_labels.iter() {
            rewrite_def(def_name, &mut state);
        }
    }

    program.defs = state.lifted_statements;
    program
}
