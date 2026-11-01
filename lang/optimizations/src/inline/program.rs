use axcut::syntax::{ID, Prog};
use std::collections::HashMap;

use crate::inline::{def, statements::count_used_functions};

pub fn inline(prog: &mut Prog) {
    let mut occurences = count_occurences(prog);
    inline_defs(prog, &mut occurences);
    remove_unused(prog, &occurences);
}

fn count_occurences(prog: &Prog) -> HashMap<ID, i32> {
    let mut occurences: HashMap<ID, i32> = HashMap::with_capacity(prog.defs.len());
    for def in prog.defs.iter() {
        count_used_functions(&def.body, &mut occurences);
    }
    occurences
}

fn inline_defs(prog: &mut Prog, occurences: &mut HashMap<ID, i32>) {
    for def in prog.defs.iter_mut() {
        def::inline(def, occurences);
    }
}

fn remove_unused(prog: &mut Prog, occurences: &HashMap<ID, i32>) {
    unimplemented!()
}