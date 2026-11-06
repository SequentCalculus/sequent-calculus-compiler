use axcut::syntax::{Def, Identifier, Prog};
use std::collections::HashMap;

use crate::inline::{Inline, Mark, Occurrences};

/// Inlines all definitions with 0 or 1 textual occurrences
/// Inlines (/removes) all bindings that are not used
pub fn inline(prog: &mut Prog) {
    let defs = std::mem::take(&mut prog.defs);
    let mut def_map: HashMap<Identifier, Def> = HashMap::with_capacity(defs.len());
    let mut def_iter = defs.into_iter();
    let entry = def_iter.next().expect("Entry point should be defined");
    let entry_ident = entry.name.clone();
    def_map.insert(entry_ident.clone(), entry);
    def_map.extend(def_iter.map(|def| (def.name.clone(), def)));

    let entry = def_map.get(&entry_ident).unwrap();
    let mut occurrences: HashMap<Identifier, Mark> = HashMap::new();
    entry.body.label_occurrences(&def_map, &mut occurrences);

    let mut entry = def_map.remove(&entry_ident).unwrap();
    let mut retain = Vec::new();
    entry.body = entry.body.inline(&mut retain, &mut def_map, &occurrences).result;

    // restore the invariant that main is at position 0
    prog.defs.push(entry);
    prog.defs.extend(retain);
}