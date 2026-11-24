use std::{collections::HashSet, rc::Rc};

use axcut::syntax::{ID, Statement};
use replace_with::replace_with_or_abort; 

pub trait DeadBindings {
    fn dead_bindings(self, free: &mut HashSet<ID>) -> Self;
}

impl DeadBindings for Statement {
    fn dead_bindings(self, free: &mut HashSet<ID>) -> Statement {
        // The signature is unusual, but works due to the uniqueness of identifiers,
        // which is why we also never remove from `free`. This signature makes
        // the computation itself way easier
        match self {
            Statement::Substitute(_) => panic!("Dead bindings should be done before linearization"),
            Statement::Literal(mut lit) => {
                lit.next = lit.next.dead_bindings(free);
                if free.contains(&lit.var.id) { lit.into() }
                else { Rc::unwrap_or_clone(lit.next).into() }
            }
            Statement::Let(mut l) => {
                l.next = l.next.dead_bindings(free);
                if free.contains(&l.var.id) {
                    free.extend(l.args.iter_ids());
                    l.into()
                } else { Rc::unwrap_or_clone(l.next).into() }
            }
            Statement::Op(mut op) => {
                op.next = op.next.dead_bindings(free);
                if free.contains(&op.var.id) {
                    free.extend([op.fst.id, op.snd.id]);
                    op.into()
                } else { Rc::unwrap_or_clone(op.next).into() }
            },
            Statement::Create(mut create) => {
                create.next = create.next.dead_bindings(free);
                if free.contains(&create.var.id) {
                    for clause in create.clauses.iter_mut() {
                        replace_with_or_abort(&mut clause.body, |body| body.dead_bindings(free));
                    }
                    create.into()
                } else { Rc::unwrap_or_clone(create.next).into() }
            }
            Statement::Switch(mut switch) => {
                for clause in switch.clauses.iter_mut() {
                    replace_with_or_abort(&mut clause.body, |body| body.dead_bindings(free));
                }
                free.insert(switch.var.id);
                switch.into()
            }
            Statement::IfC(mut ifc) => {
                ifc.thenc = ifc.thenc.dead_bindings(free);
                ifc.elsec = ifc.elsec.dead_bindings(free);
                free.insert(ifc.fst.id);
                ifc.snd.clone().map(|snd| free.insert(snd.id));
                ifc.into()
            }
            Statement::PrintI64(mut print) => {
                print.next = print.next.dead_bindings(free);
                free.insert(print.var.id);
                print.into()
            },
            Statement::Call(call) => {
                free.extend(call.args.iter_ids());
                call.into()
            }
            Statement::Exit(exit) => {
                free.insert(exit.var.id);
                exit.into()
            }
            Statement::Invoke(invoke) => {
                free.insert(invoke.var.id);
                free.extend(invoke.args.iter_ids());
                invoke.into()
            }
        }
    }
}

impl DeadBindings for Rc<Statement> {
    fn dead_bindings(self, free: &mut HashSet<ID>) -> Rc<Statement> {
        Rc::new(Rc::unwrap_or_clone(self).dead_bindings(free))
    }
}
