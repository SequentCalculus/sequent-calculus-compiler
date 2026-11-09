use std::{collections::{HashMap, HashSet}, rc::Rc};

use axcut::{syntax::{Def, Identifier, Statement}, traits::free_vars::FreeVars};

use crate::inline::{Inline, InlineResult, Mark, Occurrences};


mod call;
mod create;
mod ifc;
mod literal;
mod r#let;
mod op;
mod print;
mod switch;

fn subst_panic<T>() -> T { panic!("Inlining should be done before linearization") }

impl Occurrences for Statement {
    fn label_occurrences(
        &self,
        defs: &HashMap<Identifier, Def>,
        occurrences: &mut HashMap<Identifier, Mark>,
    ) {
        match self {
            Statement::Call(call) => call.label_occurrences(defs, occurrences),
            Statement::Substitute(_) => subst_panic(),
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

impl Inline for Statement {
    fn inline(
        self,
        retain: &mut Vec<Def>,
        to_visit: &mut HashMap<Identifier, Def>,
        occurrences: &HashMap<Identifier, Mark>,
    ) -> InlineResult {
        match self {
            Statement::Substitute(_) => subst_panic(),
            Statement::Call(call) => call.inline(retain, to_visit, occurrences),
            Statement::Switch(switch) => switch.inline(retain, to_visit, occurrences),
            Statement::Create(create) => create.inline(retain, to_visit, occurrences),
            Statement::IfC(ifc) => ifc.inline(retain, to_visit, occurrences), 
            Statement::Let(l) => l.inline(retain, to_visit, occurrences),
            Statement::Literal(lit) => lit.inline(retain, to_visit, occurrences),
            Statement::Op(op) => op.inline(retain, to_visit, occurrences),
            Statement::PrintI64(print) => print.inline(retain, to_visit, occurrences),
            Statement::Exit(exit) => {
                let mut free_vars = HashSet::new();
                let annotated = exit.free_vars(&mut free_vars);
                InlineResult {
                    result: annotated.into(),
                    free_vars,
                }
            }
            Statement::Invoke(invoke) => {
                let mut free_vars = HashSet::new();
                let annotated = invoke.free_vars(&mut free_vars);
                InlineResult {
                    result: annotated.into(),
                    free_vars,
                }
            }
        }
    }
}

impl Inline for Rc<Statement> {
    fn inline(
        self,
        retain: &mut Vec<Def>,
        to_visit: &mut HashMap<Identifier, Def>,
        occurrences: &HashMap<Identifier, Mark>,
    ) -> InlineResult {
        Rc::unwrap_or_clone(self).inline(retain, to_visit, occurrences)
    }
}

