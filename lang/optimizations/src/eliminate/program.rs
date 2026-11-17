use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::Prog;
use replace_with::replace_with_or_abort;
use std::collections::HashMap;

pub fn eliminate(program: &mut Prog) {
    let mut defs = std::mem::take(&mut program.defs);
    let mut state = EliminateState {
        global_switches: HashMap::new(),
        let_bindings: HashMap::new(),
        create_bindings: HashMap::new(),
        max_id: program.max_id,
    };

    state.lift_global_switches(&mut defs);

    // After doing cut elimination in the original `Def`s, we repeatedly
    // do this for all the lifted `Clause`s located at the end of the `Def` vector, if any
    let mut number_of_rewritten_defs = 0;
    while number_of_rewritten_defs < defs.len() {
        let number_of_defs = defs.len();
        for def_position in number_of_rewritten_defs..number_of_defs {
            state.let_bindings = HashMap::new();
            state.create_bindings = HashMap::new();
            let mut lifted = Vec::new();
            replace_with_or_abort(&mut defs[def_position].body, |body| {
                body.eliminate(&mut state, &mut lifted)
            });
            defs.extend(lifted);
        }
        number_of_rewritten_defs = number_of_defs;
    }

    program.max_id = state.max_id;
    program.defs = defs;
}
