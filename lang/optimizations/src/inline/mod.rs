use std::{cell::Cell, rc::Rc};

use axcut::syntax::{Def, Statement};

pub mod program;

#[derive(Clone)]
struct Budget {
    remaining: Rc<Cell<i32>>
}

impl Budget {
    fn new(init: i32) -> Budget {
        Budget{ remaining: Rc::new(Cell::new(init)) }
    }

    fn decr(&self) {
        self.remaining.replace(self.remaining.get() - 1);
    }

    fn get(&self) -> i32 {
        self.remaining.get()
    }
}

struct DefInfo {
    def: Def,
    len: i32
}

trait Length {
    fn len(&self) -> i32;
}

impl Length for Statement {
    fn len(&self) -> i32 {
        match self {
            Statement::Substitute(subst) => 1 + subst.next.len(),
            Statement::Create(create) => {
                let clause_len: i32 = create.clauses.iter().map(|c| c.body.len()).sum();
                1 + clause_len + create.next.len()
            }
            Statement::Switch(switch) => {
                let clause_len: i32 = switch.clauses.iter().map(|c| c.body.len()).sum(); 
                1 + clause_len
            }
            Statement::IfC(ifc) => 1 + ifc.thenc.len() + ifc.elsec.len(),
            Statement::Let(l) => 1 + l.next.len(),
            Statement::Literal(lit) => 1 + lit.next.len(),
            Statement::Op(op) => 1 + op.next.len(),
            Statement::PrintI64(print) => 1 + print.next.len(),
            Statement::Call(_) | Statement::Exit(_) | Statement::Invoke(_) => 1
        }
    }
}