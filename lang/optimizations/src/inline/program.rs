use axcut::syntax::{Def, Prog};
use std::collections::HashMap;

use super::statements::Inline;

pub enum Mark {
    None,
    Once,
    Retain,
}

pub fn inline(prog: &mut Prog) {
    let defs = std::mem::take(&mut prog.defs);
    let mut occurrences: HashMap<String, Mark> = HashMap::with_capacity(defs.len());
    let mut def_map: HashMap<String, Def> = HashMap::with_capacity(defs.len());
    occurrences.extend(defs.iter().enumerate().map(|(i, d)| {
        let mark = if i == 0 {
            Mark::Retain
        } else { Mark::None };
        (d.name.name.clone(), mark)
    }));

    let mut def_iter = defs.into_iter();
    let entry = def_iter.next().expect("Entry point should be defined");
    let entry_name = String::clone(&entry.name.name);
    def_map.insert(entry_name.clone(), entry);
    def_map.extend(def_iter.map(|def| (def.name.name.clone(), def)));

    let entry = def_map.get(&entry_name).unwrap();
    entry.body.count_occurences(&def_map, &mut occurrences);

    let mut entry = def_map.remove(&entry_name).unwrap();
    println!("Entry is {}", entry.name.name);
    let mut retain = Vec::new();
    entry.body = entry.body.inline(&mut retain, &mut def_map, &occurrences);
    retain.push(entry);
    prog.defs = retain;
    for def in prog.defs.iter() {
        println!("Retained: {}", def.name.name);
    }
}