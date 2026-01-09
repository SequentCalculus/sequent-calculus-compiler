use crate::{
    cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState, DefInfo, Mark},
    def::rewrite_def,
    rewrite::RewriteState,
};
use axcut::syntax::{Def, Prog};
use std::collections::{HashMap, HashSet};

pub const MAX_RUNS: usize = usize::MAX;

pub fn cleanup_inline_defs(defs: Vec<Def>) -> Vec<Def> {
    let def_map = defs
        .iter()
        .enumerate()
        .map(|(position, def)| {
            let mark = if position == 0 {
                // we always retain the entry point
                Mark::Retain
            } else {
                Mark::None
            };
            (def.name.clone(), DefInfo { position, mark })
        })
        .collect();
    let mut cleanup_inline_state = CleanupInlineState {
        defs,
        def_map,
        used_vars: HashSet::default(),
        current_def_mark: Mark::Once,
    };
    // we traverse the call graph starting at the entry point, gathering information and inlining
    // trivial definitions
    let called_def_body = std::mem::take(&mut cleanup_inline_state.defs[0].body);
    cleanup_inline_state.defs[0].body =
        called_def_body.cleanup_inline_gather(&mut cleanup_inline_state);

    // now we inline all functions called exactly once into those definitions we retain
    let number_of_defs = cleanup_inline_state.defs.len();
    let mut retained_defs = Vec::with_capacity(number_of_defs);
    for position in 0..number_of_defs {
        let def = &mut cleanup_inline_state.defs[position];
        let name = def.name.clone();
        let mark = cleanup_inline_state
            .def_map
            .get(&name)
            .unwrap_or_else(|| panic!("Definition {name} must be in the map of definitions"))
            .mark;
        if mark == Mark::Retain {
            cleanup_inline_state.used_vars = std::mem::take(&mut def.used_vars);
            let context = std::mem::take(&mut def.context.bindings).into();
            let body = std::mem::take(&mut def.body).cleanup_inline(&mut cleanup_inline_state);
            let def = Def {
                name,
                context,
                used_vars: cleanup_inline_state.used_vars,
                body,
            };
            retained_defs.push(def);
        }
    }

    retained_defs
}

pub fn rewrite_prog(mut program: Prog) -> Prog {
    // we inline once at the beginning to potentially find more rewrites and avoid rewriting dead
    // code in the first pass
    let defs = cleanup_inline_defs(std::mem::take(&mut program.defs));
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
        // rewritten once per pass
        let mut number_of_rewritten_defs = 0;
        while state.defs.len() > number_of_rewritten_defs {
            let number_of_defs = state.defs.len();
            for def_position in number_of_rewritten_defs..number_of_defs {
                rewrite_def(def_position, &mut state);
            }
            number_of_rewritten_defs = number_of_defs;
        }

        state.defs = cleanup_inline_defs(std::mem::take(&mut state.defs));
    }

    program.defs = state.defs;
    program
}
