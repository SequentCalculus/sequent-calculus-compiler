use crate::{OptimizationStats, def::rewrite_def, rewrite::RewriteState};
use axcut::syntax::Prog;
use std::{
    collections::{HashMap, HashSet},
    mem::take,
};

pub fn rewrite_prog(mut program: Prog, max_runs: u64) -> (Prog, OptimizationStats) {
    // we thread the set of labels of top-level functions through the translation, because we need
    // to generate fresh labels when we lift statements
    let used_labels = program.defs.iter().map(|def| def.name.clone()).collect();
    let defs = take(&mut program.defs);
    let mut state = RewriteState {
        used_labels,
        lifted_statements: defs,
        current_label: "main".to_owned(),
        current_used_vars: HashSet::new(),
        let_bindings: HashMap::new(),
        create_bindings: HashMap::new(),
        new_changes: true,
    };

    let mut performed_runs = 0;
    while state.new_changes && performed_runs < max_runs {
        state.new_changes = false;
        performed_runs += 1;
        let mut current_labels = state.used_labels.iter().cloned().collect::<Vec<_>>();
        current_labels.sort();
        for def_name in current_labels.iter() {
            rewrite_def(def_name, &mut state);
        }
    }

    program.defs = state.lifted_statements;
    let stats = OptimizationStats {
        num_passes: performed_runs,
    };
    (program, stats)
}
