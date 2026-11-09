use axcut::syntax::{Def, Identifier, Prog};
use std::collections::HashMap;

use crate::inline::{Inline, Mark, Occurrences};

/// Inlines all definitions with 0 or 1 textual occurrences
/// Inlines (/removes) all bindings that are not used
pub fn inline(prog: &mut Prog) {
    let original_order: HashMap<Identifier, usize> =
        prog.defs.iter().enumerate().map(|(i, d)| (d.name.clone(), i)).collect();
    let defs = std::mem::take(&mut prog.defs);
    let mut def_map: HashMap<Identifier, Def> = HashMap::with_capacity(defs.len());
    let mut def_iter = defs.into_iter();
    let entry = def_iter.next().expect("Entry point should be defined");
    let entry_ident = entry.name.clone();
    def_map.insert(entry_ident.clone(), entry);
    def_map.extend(def_iter.map(|def| (def.name.clone(), def)));

    let entry = def_map.get(&entry_ident).unwrap();
    let mut occurrences: HashMap<Identifier, Mark> = HashMap::new();
    occurrences.insert(entry_ident.clone(), Mark::Retain);
    entry.body.label_occurrences(&def_map, &mut occurrences);

    let mut entry = def_map.remove(&entry_ident).unwrap();
    entry.body = entry
        .body
        .inline(&mut prog.defs, &mut def_map, &occurrences)
        .result;
    prog.defs.push(entry);

    // restore the order of definitions
    prog.defs.sort_unstable_by_key(|d| original_order[&d.name]);
}
