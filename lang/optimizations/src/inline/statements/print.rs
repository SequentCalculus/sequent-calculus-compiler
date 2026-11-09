use std::collections::HashMap;

use axcut::syntax::{Def, Identifier, statements::PrintI64};

use crate::inline::{Inline, InlineResult, Mark};

impl Inline for PrintI64 {
    fn inline(
        mut self,
        retain: &mut Vec<Def>,
        to_visit: &mut HashMap<Identifier, Def>,
        occurrences: &HashMap<Identifier, Mark>,
    ) -> InlineResult {
        let InlineResult {
            result,
            mut free_vars,
        } = self.next.inline(retain, to_visit, occurrences);
        self.next = result.into();
        free_vars.insert(self.var.id);
        InlineResult {
            result: self.into(),
            free_vars,
        }
    }
}
