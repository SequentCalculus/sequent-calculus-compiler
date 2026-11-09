use std::collections::{HashMap, HashSet};

use axcut::syntax::{Def, ID, Identifier, Statement};

pub mod program;
pub mod statements;

pub enum Mark {
    Once,
    Retain,
}

pub trait Occurrences {
    fn label_occurrences(
        &self,
        defs: &HashMap<Identifier, Def>,
        occurrences: &mut HashMap<Identifier, Mark>,
    );
}

pub struct InlineResult {
    pub result: Statement,
    pub free_vars: HashSet<ID>,
}

pub trait Inline {
    fn inline(
        self,
        retain: &mut Vec<Def>,
        to_visit: &mut HashMap<Identifier, Def>,
        occurrences: &HashMap<Identifier, Mark>,
    ) -> InlineResult;
}
