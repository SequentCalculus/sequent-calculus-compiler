use std::{collections::HashMap, fmt::Debug, rc::Rc};

use axcut::syntax::{Def, Statement};

use super::program::Mark;

mod call;

pub trait Inline {
    type Target;
    fn count_occurences(&self, defs: &HashMap<String, Def>, occurrences: &mut HashMap<String, Mark>);
    fn inline(self, retain: &mut Vec<Def>, to_visit: &mut HashMap<String, Def>, occurrences: &HashMap<String, Mark>) -> Self::Target;
}

impl Inline for Statement {
    type Target = Statement;
    fn count_occurences(&self, defs: &HashMap<String, Def>, occurrences: &mut HashMap<String, Mark>) {
        match self {
            Statement::Call(call) => call.count_occurences(defs, occurrences),
            Statement::Substitute(subst) => subst.next.count_occurences(defs, occurrences),
            Statement::Let(l) => l.next.count_occurences(defs, occurrences),
            Statement::Switch(switch) => {
                for clause in switch.clauses.iter() {
                    clause.body.count_occurences(defs, occurrences);
                }
            }
            Statement::Create(create) => {
                for clause in create.clauses.iter() {
                    clause.body.count_occurences(defs, occurrences);
                }
                create.next.count_occurences(defs, occurrences);
            }
            Statement::Literal(lit) => lit.next.count_occurences(defs, occurrences),
            Statement::Op(op) => op.next.count_occurences(defs, occurrences),
            Statement::PrintI64(print) => print.next.count_occurences(defs, occurrences),
            Statement::IfC(ifc) => {
                ifc.thenc.count_occurences(defs, occurrences);
                ifc.elsec.count_occurences(defs, occurrences);
            }
            Statement::Invoke(_) | Statement::Exit(_) => {}
        }
    }
    
    fn inline(self, retain: &mut Vec<Def>, to_visit: &mut HashMap<String, Def>, occurrences: &HashMap<String, Mark>) -> Statement {
        match self {
            Statement::Call(call) => call.inline(retain, to_visit, occurrences),
            Statement::Switch(mut switch) => {
                switch.clauses = std::mem::take(&mut switch.clauses)
                    .into_iter().map(|mut c| {
                        c.body = c.body.inline(retain, to_visit, occurrences);
                        c
                    }).collect();
                switch.into()
            }
            Statement::Create(mut create) => {
                create.clauses = std::mem::take(&mut create.clauses)
                    .into_iter().map(|mut c| {
                        c.body = c.body.inline(retain, to_visit, occurrences);
                        c
                    }).collect();
                create.next = create.next.inline(retain, to_visit, occurrences);
                create.into()
            },
            Statement::IfC(mut ifc) => {
                ifc.thenc = ifc.thenc.inline(retain, to_visit, occurrences);
                ifc.elsec = ifc.elsec.inline(retain, to_visit, occurrences);
                ifc.into()
            },
            Statement::Substitute(mut subst) => {
                subst.next = subst.next.inline(retain, to_visit, occurrences);
                subst.into()
            }
            Statement::Let(mut l) => {
                l.next = l.next.inline(retain, to_visit, occurrences);
                l.into()
            }
            Statement::Literal(mut lit) => {
                lit.next = lit.next.inline(retain, to_visit, occurrences);
                lit.into()
            }
            Statement::Op(mut op) => {
                op.next = op.next.inline(retain, to_visit, occurrences);
                op.into()
            }
            Statement::PrintI64(mut print) => {
                print.next = print.next.inline(retain, to_visit, occurrences);
                print.into()
            }
            Statement::Exit(_) | Statement::Invoke(_) => self
        }
    }
}

impl<T: Inline + Clone + Debug> Inline for Rc<T> {
    type Target = Rc<T::Target>;
    fn count_occurences(&self, defs: &HashMap<String, Def>, occurrences: &mut HashMap<String, Mark>) {
        self.as_ref().count_occurences(defs, occurrences);
    }

    fn inline(self, retain: &mut Vec<Def>, to_visit: &mut HashMap<String, Def>, occurrences: &HashMap<String, Mark>) -> Rc<T::Target> {
        Rc::new(Rc::unwrap_or_clone(self).inline(retain, to_visit, occurrences))
    }
}