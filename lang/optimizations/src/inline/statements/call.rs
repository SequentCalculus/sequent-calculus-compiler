use core::panic;
use std::collections::HashMap;

use axcut::{syntax::{ID, Identifier, Statement, statements::Call}, traits::substitution::Subst};

use crate::inline::program::Mark;

use super::Inline;

impl Inline for Call {
    type Target = Statement;
    fn count_occurences(&self, defs: &HashMap<String, axcut::syntax::Def>, occurrences: &mut HashMap<String, Mark>) {
        let callee_name = &self.label.name;
        match occurrences.get(callee_name).unwrap() {
            //not yet visited
            Mark::None => {
                let def = defs.get(callee_name).unwrap();
                occurrences.insert(callee_name.clone(), Mark::Once);
                def.body.count_occurences(defs, occurrences);
            }
            // already visited
            Mark::Once => { occurrences.insert(callee_name.clone(), Mark::Retain); },
            Mark::Retain => {}
        }
    }

    fn inline(self, retain: &mut Vec<axcut::syntax::Def>, to_visit: &mut HashMap<String, axcut::syntax::Def>, occurrences: &HashMap<String, Mark>) -> Statement {
        let callee_name = &self.label.name;
        match occurrences.get(callee_name).expect("Label needs to exist") {
            Mark::None => {
                panic!("Unused label used")
            }
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
                    None => self.into(),
                    Some(mut def) => {
                        def.body = def.body.inline(retain, to_visit, occurrences);
                        retain.push(def);
                        self.into()
                    }
                }
            }
        }
    }
}