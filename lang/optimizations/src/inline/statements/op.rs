use std::collections::HashMap;

use axcut::syntax::{Def, Identifier, statements::Op};

use crate::inline::{Inline, InlineResult, Mark};

impl Inline for Op {
    fn inline(
        mut self,
        retain: &mut Vec<Def>,
        to_visit: &mut HashMap<Identifier, Def>,
        occurrences: &HashMap<Identifier, Mark>,
    ) -> InlineResult
    {
        let InlineResult {
            result,
            mut free_vars,
        } = self.next.inline(retain, to_visit, occurrences);
        if !free_vars.contains(&self.var.id) {
            InlineResult { result, free_vars }
        } else {
            self.next = result.into();
            free_vars.insert(self.fst.id);
            free_vars.insert(self.snd.id);
            free_vars.remove(&self.var.id);
            InlineResult {
                result: self.into(),
                free_vars,
            }
        }
    }
}