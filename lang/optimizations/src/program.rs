use crate::{def::rewrite_def, errors::Error};
use axcut::syntax::Prog;

pub const MAX_RUNS: u64 = 10;

pub fn rewrite_prog(mut program: Prog) -> Result<Prog, Error> {
    // we thread the set of labels of top-level functions through the translation, because we need
    // to generate fresh labels when we lift statements
    let mut used_labels = program.defs.iter().map(|def| def.name.clone()).collect();

    let mut performed_runs = 0;
    let mut new_changes = true;
    while new_changes && performed_runs < MAX_RUNS {
        new_changes = false;
        performed_runs += 1;
        let mut new_defs = Vec::with_capacity(program.defs.len());
        for def in program.defs {
            new_defs.extend(rewrite_def(def, &mut used_labels, &mut new_changes)?);
        }
        program.defs = new_defs;
    }
    Ok(program)
}
