use std::collections::{HashMap, HashSet};

use axcut::syntax::{Def, Identifier, Prog};
use replace_with::replace_with_or_abort;

use super::{dead_bindings::DeadBindings, label_occurences::LabelOccurrences};


pub fn dead_code(prog: &mut Prog) {
    prog.use_defs(|defs| {
        let indices: HashMap<Identifier, usize> =
            defs.iter().enumerate().map(|(i, d)| (d.name.clone(), i)).collect();
        let mut used_indices = HashSet::from([0]);
        let entry = &defs[0];
        entry.body.label_occurrences(&defs, &indices, &mut used_indices);

        let mut used_defs: Vec<Def> = defs.into_iter().enumerate()
            .filter_map(|(i, d)| {
                if used_indices.contains(&i) { Some(d) }
                else { None }
            }).collect();

        for def in used_defs.iter_mut() {
            replace_with_or_abort(&mut def.body, |body| body.dead_bindings(&mut HashSet::new()));
        }

        used_defs
    });
}