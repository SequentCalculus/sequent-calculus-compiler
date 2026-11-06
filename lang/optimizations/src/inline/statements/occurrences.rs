use std::collections::HashMap;

use axcut::syntax::{Def, Identifier, Statement};

use crate::inline::{Mark, Occurrences};

impl Occurrences for Statement {
    fn label_occurrences(&self, defs: &HashMap<Identifier, Def>, occurrences: &mut HashMap<Identifier, Mark>) {
        match self {
            Statement::Call(call) => call.label_occurrences(defs, occurrences),
            Statement::Substitute(subst) => subst.next.label_occurrences(defs, occurrences),
            Statement::Let(l) => l.next.label_occurrences(defs, occurrences),
            Statement::Switch(switch) => {
                for clause in switch.clauses.iter() {
                    clause.body.label_occurrences(defs, occurrences);
                }
            }
            Statement::Create(create) => {
                for clause in create.clauses.iter() {
                    clause.body.label_occurrences(defs, occurrences);
                }
                create.next.label_occurrences(defs, occurrences);
            }
            Statement::Literal(lit) => lit.next.label_occurrences(defs, occurrences),
            Statement::Op(op) => op.next.label_occurrences(defs, occurrences),
            Statement::PrintI64(print) => print.next.label_occurrences(defs, occurrences),
            Statement::IfC(ifc) => {
                ifc.thenc.label_occurrences(defs, occurrences);
                ifc.elsec.label_occurrences(defs, occurrences);
            }
            Statement::Invoke(_) | Statement::Exit(_) => {}
        }
    }
}