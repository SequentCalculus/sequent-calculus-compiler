use crate::{def::rewrite_def, rewrite::RewriteState};
use axcut::syntax::Prog;
use std::collections::{HashMap, HashSet};

pub const MAX_RUNS: usize = usize::MAX;

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
    while state.new_changes && performed_runs < MAX_RUNS {
        state.new_changes = false;
        performed_runs += 1;

        // after rewriting the original `Def`s, we repeatedly rewrite all the lifted `Clause`s,
        // located at the end of the `Def` vector, if any, to ensure that all statements are
        // rewritten once
        let mut number_of_rewritten_defs = 0;
        while state.defs.len() > number_of_rewritten_defs {
            let number_of_defs = state.defs.len();
            for def_position in number_of_rewritten_defs..number_of_defs {
                rewrite_def(def_position, &mut state);
            }
            number_of_rewritten_defs = number_of_defs;
        }
    }

    program.defs = state.defs;
    program
}
