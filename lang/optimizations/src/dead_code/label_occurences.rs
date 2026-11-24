use std::collections::{HashMap, HashSet};

use axcut::syntax::{Def, Identifier, Statement};


pub trait LabelOccurrences {
    fn label_occurrences(
        &self,
        defs: &Vec<Def>,
        indices: &HashMap<Identifier, usize>,
        used_indices: &mut HashSet<usize>,
    );
}

impl LabelOccurrences for Statement {
    fn label_occurrences(
        &self,
        defs: &Vec<Def>,
        indices: &HashMap<Identifier, usize>,
        used_indices: &mut HashSet<usize>,
    ) {
        match self {
            Statement::Call(call) => {
                let callee = &call.label;
                let index = indices[callee];
                if !used_indices.contains(&index) {
                    used_indices.insert(index);
                    let def = &defs[index];
                    def.body.label_occurrences(defs, indices, used_indices);
                }
            }
            Statement::Substitute(subst) => subst.next.label_occurrences(defs, indices, used_indices),
            Statement::Let(l) => l.next.label_occurrences(defs, indices, used_indices),
            Statement::Switch(switch) => {
                for clause in switch.clauses.iter() {
                    clause.body.label_occurrences(defs, indices, used_indices);
                }
            }
            Statement::Create(create) => {
                for clause in create.clauses.iter() {
                    clause.body.label_occurrences(defs, indices, used_indices);
                }
                create.next.label_occurrences(defs, indices, used_indices);
            }
            Statement::Literal(lit) => lit.next.label_occurrences(defs, indices, used_indices),
            Statement::Op(op) => op.next.label_occurrences(defs, indices, used_indices),
            Statement::PrintI64(print) => print.next.label_occurrences(defs, indices, used_indices),
            Statement::IfC(ifc) => {
                ifc.thenc.label_occurrences(defs, indices, used_indices);
                ifc.elsec.label_occurrences(defs, indices, used_indices);
            }
            Statement::Invoke(_) | Statement::Exit(_) => {}
        }
    }
}