use crate::def::rewrite_def;
use axcut::syntax::Prog;
use std::collections::{HashMap, HashSet};

pub const MAX_RUNS: u64 = u64::MAX;

pub fn rewrite_prog(mut program: Prog) -> Prog {
    let defs = std::mem::take(&mut program.defs);
    // we thread the set of labels of top-level functions through the translation, because we need
    // to generate fresh labels when we lift statements
    let used_labels = defs.iter().map(|def| def.name.clone()).collect();
    let mut state = RewriteState {
        used_labels,
        defs,
        current_label: "".to_owned(),
        current_used_vars: HashSet::new(),
        let_bindings: HashMap::new(),
        create_bindings: HashMap::new(),
        new_changes: true,
    };

    let mut performed_runs = 0;
    let mut new_changes = true;
    while new_changes && performed_runs < MAX_RUNS {
        new_changes = false;
        performed_runs += 1;
        for def_position in 0..state.defs.len() {
            rewrite_def(def_position, &mut state);
        }
    }

<<<<<<< HEAD
=======
    program.defs = state.defs;
>>>>>>> 8805f85 (Refactor, streamline, and fix bugs)
    program
}
