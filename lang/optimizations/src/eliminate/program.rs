use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::Prog;
use std::collections::HashMap;

pub fn eliminate(program: &mut Prog) {
    let mut state = EliminateState {
        defs: std::mem::take(&mut program.defs),
        global_switches: HashMap::new(),
        let_bindings: HashMap::new(),
        create_bindings: HashMap::new(),
        max_id: &mut program.max_id
    };

    state.lift_global_switches();

    // After doing cut elimination in the original `Def`s, we repeatedly
    // do this for all the lifted `Clause`s located at the end of the `Def` vector, if any
    let mut number_of_rewritten_defs = 0;
    while number_of_rewritten_defs < state.defs.len() {
        let number_of_defs = state.defs.len();
        for def_position in number_of_rewritten_defs..number_of_defs {
            let body = state.defs[def_position].clone().body; // TODO
            state.let_bindings = HashMap::new();
            state.create_bindings = HashMap::new();
            body.eliminate(&mut state);
        }
        number_of_rewritten_defs = number_of_defs;
    }

    program.defs = state.remove_defs();
}
