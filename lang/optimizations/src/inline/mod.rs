use std::collections::{HashMap, HashSet};

use axcut::syntax::{Def, ID, Identifier, Statement};

pub mod program;
mod statements;

enum Mark {
    Once,
    Retain,
}

trait Occurrences {
    fn label_occurrences(
        &self,
        defs: &HashMap<Identifier, Def>,
        occurrences: &mut HashMap<Identifier, Mark>,
    );
}

struct InlineResult {
    result: Statement,
    free_vars: HashSet<ID>,
}

trait Inline {
    fn inline(
        self,
        retain: &mut Vec<Def>,
        to_visit: &mut HashMap<Identifier, Def>,
        occurrences: &HashMap<Identifier, Mark>,
    ) -> InlineResult;
}
