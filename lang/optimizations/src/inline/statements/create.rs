use std::collections::HashMap;

use axcut::syntax::{Def, Identifier, statements::Create};

use crate::inline::{Inline, InlineResult, Mark};

impl Inline for Create {
    fn inline(
        mut self,
        retain: &mut Vec<Def>,
        to_visit: &mut HashMap<Identifier, Def>,
        occurrences: &HashMap<Identifier, Mark>,
    ) -> InlineResult {
        let InlineResult { result, free_vars } =
            self.next.inline(retain, to_visit, occurrences);
        if !free_vars.contains(&self.var.id) {
            InlineResult { result, free_vars }
        } else {
            let mut all_free_vars = free_vars;
            self.next = result.into();
            self.clauses = std::mem::take(&mut self.clauses)
                .into_iter()
                .map(|mut c| {
                    let InlineResult {
                        result,
                        mut free_vars,
                    } = c.body.inline(retain, to_visit, occurrences);
                    for bind in c.context.bindings.iter() {
                        free_vars.remove(&bind.var.id);
                    }
                    all_free_vars.extend(free_vars);
                    c.body = result.into();
                    c
                })
                .collect();
            all_free_vars.remove(&self.var.id);
            InlineResult {
                result: self.into(),
                free_vars: all_free_vars,
            }
        }
    }
}