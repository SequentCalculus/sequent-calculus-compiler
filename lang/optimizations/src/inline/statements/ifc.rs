use std::collections::{HashMap, HashSet};

use axcut::syntax::{Def, Identifier, statements::IfC};

use crate::inline::{Inline, InlineResult, Mark};

impl Inline for IfC {
    fn inline(
        mut self,
        retain: &mut Vec<Def>,
        to_visit: &mut HashMap<Identifier, Def>,
        occurrences: &HashMap<Identifier, Mark>,
    ) -> InlineResult {
        let mut all_free_vars = HashSet::new();
        let InlineResult { result, free_vars } = self.thenc.inline(retain, to_visit, occurrences);
        all_free_vars.extend(free_vars);
        self.thenc = result.into();
        let InlineResult { result, free_vars } = self.elsec.inline(retain, to_visit, occurrences);
        all_free_vars.extend(free_vars);
        self.elsec = result.into();
        all_free_vars.insert(self.fst.id);
        self.snd.as_ref().inspect(|n| {
            all_free_vars.insert(n.id);
        });
        InlineResult {
            result: self.into(),
            free_vars: all_free_vars,
        }
    }
}
