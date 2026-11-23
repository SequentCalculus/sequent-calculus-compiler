use axcut::syntax::Prog;


pub fn dead_code(prog: &mut Prog) {
    /*let entry = def_map.get(&entry_ident).unwrap();
    let mut occurrences: HashMap<Identifier, Mark> = HashMap::new();
    occurrences.insert(entry_ident.clone(), Mark::Retain);
    entry.body.label_occurrences(&def_map, &mut occurrences);

    let default_budget = 1000;
    let queue: VecDeque<(Statement, Cell<i64>)> =
        used_defs.map(|(_, def)| (&mut def.body, Cell::new(default_budget))).collect();

    inline_bfs(&def_map, queue);
    let mut entry = def_map.remove(&entry_ident).unwrap();
    entry.body = entry
        .body
        .inline(&mut prog.defs, &mut def_map, &occurrences)
        .result;
    prog.defs.push(entry);*/
}