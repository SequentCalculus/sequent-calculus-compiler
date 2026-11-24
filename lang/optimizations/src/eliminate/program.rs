use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::Prog;
use replace_with::replace_with_or_abort;
use std::collections::HashMap;

pub fn eliminate(prog: &mut Prog) {
    prog.use_defs_and_max_id(|mut defs, mut max_id| {
        let mut state = EliminateState {
            global_switches: HashMap::new(),
            let_bindings: HashMap::new(),
            create_bindings: HashMap::new(),
            max_id: &mut max_id,
        };

        state.lift_global_switches(&mut defs);

        let mut lifted = Vec::new();
        for def in defs.iter_mut() {
            state.let_bindings = HashMap::new();
            state.create_bindings = HashMap::new();
            replace_with_or_abort(&mut def.body, |body| {
                body.eliminate(&mut state, &mut lifted)
            });
        }
        defs.extend(lifted);

        (defs, max_id)
    });
}
