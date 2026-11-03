use std::collections::{HashMap, HashSet};

use axcut::{syntax::{ID, Identifier, statements::Call}, traits::{free_vars::FreeVars, substitution::Subst}};

use crate::inline::{program::Mark, statements::InlineResult};

use super::Inline;

// TODO: Also inline if definition is leaf statement
impl Inline for Call {
    fn count_occurences(&self, defs: &HashMap<String, axcut::syntax::Def>, occurrences: &mut HashMap<String, Mark>) {
        let callee_name = &self.label.name;
        match occurrences.get_mut(callee_name) {
            //not yet visited
            None => {
                let def = defs.get(callee_name).unwrap();
                occurrences.insert(callee_name.clone(), Mark::Once);
                def.body.count_occurences(defs, occurrences);
            }
            // already visited
            Some(mark) => *mark = Mark::Retain,
        }
    }

    fn inline(self, retain: &mut Vec<axcut::syntax::Def>, to_visit: &mut HashMap<String, axcut::syntax::Def>, occurrences: &HashMap<String, Mark>) -> InlineResult {
        let callee_name = &self.label.name;
        match occurrences[callee_name] {
            Mark::Once => {
                let to_inline = to_visit.remove(callee_name).unwrap();
                let arg_iter = self.args.bindings.iter().map(|a| a.var.clone());
                let param_id_iter = to_inline.context.bindings.iter().map(|p| p.var.id);
                let subst: Vec<(ID, Identifier)> = param_id_iter.zip(arg_iter).collect();
                let substituted = to_inline.body.subst_sim(&subst);
                substituted.inline(retain, to_visit, occurrences)
            }
            Mark::Retain => {
                match to_visit.remove(callee_name) {
                    None => {
                        let mut free_vars = HashSet::new();
                        let annotated = self.free_vars(&mut free_vars);
                        InlineResult { result: annotated.into(), free_vars: free_vars }
                    },
                    Some(mut def) => {
                        let InlineResult { result, free_vars } = def.body.inline(retain, to_visit, occurrences);
                        def.body = result;
                        retain.push(def);
                        InlineResult { result: self.into(), free_vars }
                    }
                }
            }
        }
    }
}