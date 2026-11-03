use axcut::syntax::{Def, Prog};
use std::collections::HashMap;

use super::statements::Inline;

pub enum Mark {
    Once,
    Retain,
}

pub fn inline(prog: &mut Prog) {
    let mut occurrences: HashMap<String, Mark> = HashMap::new();

    let defs = std::mem::take(&mut prog.defs);
    let mut def_map: HashMap<String, Def> = HashMap::with_capacity(defs.len());
    let mut def_iter = defs.into_iter();
    let entry = def_iter.next().expect("Entry point should be defined");
    let entry_name = entry.name.name.clone();
    def_map.insert(entry_name.clone(), entry);
    def_map.extend(def_iter.map(|def| (def.name.name.clone(), def)));

    let entry = def_map.get(&entry_name).unwrap();
    entry.body.count_occurences(&def_map, &mut occurrences);

    let mut entry = def_map.remove(&entry_name).unwrap();
    entry.body = entry.body.inline(&mut prog.defs, &mut def_map, &occurrences).result;
    prog.defs.push(entry);
}