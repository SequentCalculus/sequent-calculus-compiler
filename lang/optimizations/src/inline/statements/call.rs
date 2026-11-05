use std::collections::{HashMap, HashSet};

use axcut::{syntax::{ID, Identifier, statements::Call}, traits::{free_vars::FreeVars, substitution::Subst}};

use crate::inline::{Inline, InlineResult, Mark, Occurrences};

impl Occurrences for Call {
    fn label_occurrences(&self, defs: &HashMap<String, axcut::syntax::Def>, occurrences: &mut HashMap<String, Mark>) {
        let callee_name = &self.label.name;
        match occurrences.get_mut(callee_name) {
            //not yet visited
            None => {
                let def = defs.get(callee_name).unwrap();
                occurrences.insert(callee_name.clone(), Mark::Once);
                def.body.label_occurrences(defs, occurrences);
            }
            // already visited
            Some(mark) => *mark = Mark::Retain,
        }
    }
}

impl Inline for Call {
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
                let def_to_visit = to_visit.remove(callee_name);
                let mut free_vars = HashSet::new();
                let annotated_call = self.free_vars(&mut free_vars);
                match def_to_visit {
                    None => {
                        InlineResult { result: annotated_call.into(), free_vars }
                    },
                    Some(mut def) => {
                        def.body = def.body.inline(retain, to_visit, occurrences).result;
                        retain.push(def);
                        InlineResult { result: annotated_call.into(), free_vars }
                    }
                }
            }
        }
    }
}