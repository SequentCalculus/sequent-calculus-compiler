use axcut::syntax::{ID, Statement};
use std::collections::HashMap;



pub fn count_used_functions(stmt: &Statement, occurences: &mut HashMap<ID, i32>) {
    match stmt {
        Statement::Call(call) => {
            occurences.entry(call.label.id).and_modify(|c| *c += 1).or_insert(1);
        }
        
        Statement::Switch(switch) => {
            for clause in switch.clauses.iter() {
                count_used_functions(&clause.body, occurences);
            }
        }
        Statement::Create(create) => {
            for clause in create.clauses.iter() {
                count_used_functions(&clause.body, occurences);
            }
        }
        Statement::IfC(ifc) => {
            count_used_functions(&ifc.thenc, occurences);
            count_used_functions(&ifc.elsec, occurences);
        }
        Statement::Substitute(_) | Statement::Let(_) | Statement::Invoke(_) |
        Statement::Literal(_) | Statement::Op(_) | Statement::PrintI64(_) | Statement::Exit(_) => {}
    }
}