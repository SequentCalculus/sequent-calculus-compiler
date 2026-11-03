use std::{collections::{HashMap, HashSet}, rc::Rc};

use axcut::syntax::{Def, ID, Statement};

use super::program::Mark;

mod call;

pub struct InlineResult {
    pub result: Statement,
    pub free_vars: HashSet<ID>
}

pub trait Inline {
    fn count_occurences(&self, defs: &HashMap<String, Def>, occurrences: &mut HashMap<String, Mark>);
    fn inline(self, retain: &mut Vec<Def>, to_visit: &mut HashMap<String, Def>, occurrences: &HashMap<String, Mark>) -> InlineResult;
}

impl Inline for Statement {
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
    
    fn inline(self, retain: &mut Vec<Def>, to_visit: &mut HashMap<String, Def>, occurrences: &HashMap<String, Mark>) -> InlineResult {
        match self {
            Statement::Substitute(_) => {
                panic!("Inlining should be done before linearization")
            }
            Statement::Call(call) => call.inline(retain, to_visit, occurrences),
            Statement::Switch(mut switch) => {
                let mut all_free_vars = HashSet::new();
                switch.clauses = std::mem::take(&mut switch.clauses)
                    .into_iter().map(|mut c| {
                        let InlineResult { result, free_vars } = c.body.inline(retain, to_visit, occurrences);
                        all_free_vars.extend(free_vars);
                        c.body = result.into();
                        c
                    }).collect();
                all_free_vars.insert(switch.var.id);
                InlineResult { result: switch.into(), free_vars: all_free_vars }
            }
            Statement::Create(mut create) => {
                let InlineResult { result, free_vars } = create.next.inline(retain, to_visit, occurrences);
                if !free_vars.contains(&create.var.id) {
                    InlineResult { result, free_vars }
                } else {
                    let mut all_free_vars = free_vars;
                    all_free_vars.remove(&create.var.id);
                    create.next = result.into();
                    create.clauses = std::mem::take(&mut create.clauses)
                        .into_iter().map(|mut c| {
                            let InlineResult { result, free_vars } = c.body.inline(retain, to_visit, occurrences);
                            all_free_vars.extend(free_vars);
                            c.body = result.into();
                            c
                        }).collect();
                    InlineResult { result: create.into(), free_vars: all_free_vars }
                }
            },
            Statement::IfC(mut ifc) => {
                let mut all_free_vars = HashSet::new();
                let InlineResult { result, free_vars } = ifc.thenc.inline(retain, to_visit, occurrences);
                all_free_vars.extend(free_vars);
                ifc.thenc = result.into();
                let InlineResult { result, free_vars } = ifc.elsec.inline(retain, to_visit, occurrences);
                all_free_vars.extend(free_vars);
                ifc.elsec = result.into();
                all_free_vars.insert(ifc.fst.id);
                ifc.snd.as_ref().inspect(|n| { all_free_vars.insert(n.id); });
                InlineResult { result: ifc.into(), free_vars: all_free_vars }
            },
            Statement::Let(mut l) => {
                let InlineResult { result, mut free_vars } = l.next.inline(retain, to_visit, occurrences);
                if !free_vars.contains(&l.var.id) {
                    InlineResult { result, free_vars }
                } else {
                    l.next = result.into();
                    free_vars.remove(&l.var.id);
                    InlineResult { result: l.into(), free_vars }
                }
            }
            Statement::Literal(mut lit) => {
                let InlineResult { result, mut free_vars } = lit.next.inline(retain, to_visit, occurrences);
                if !free_vars.contains(&lit.var.id) {
                    InlineResult { result, free_vars }
                } else {
                    lit.next = result.into();
                    free_vars.remove(&lit.var.id);
                    InlineResult { result: lit.into(), free_vars }
                }
            }
            Statement::Op(mut op) => {
                let InlineResult { result, mut free_vars } = op.next.inline(retain, to_visit, occurrences);
                if !free_vars.contains(&op.var.id) {
                    InlineResult { result, free_vars }
                } else {
                    op.next = result.into();
                    free_vars.remove(&op.var.id);
                    free_vars.insert(op.fst.id);
                    free_vars.insert(op.snd.id);
                    InlineResult { result: op.into(), free_vars }
                }
            }
            Statement::PrintI64(mut print) => {
                let InlineResult { result, mut free_vars } = print.next.inline(retain, to_visit, occurrences);
                print.next = result.into();
                free_vars.insert(print.var.id);
                InlineResult { result: print.into(), free_vars }
            }
            Statement::Exit(exit) => {
                let free_vars = HashSet::from([exit.var.id]);
                InlineResult { result: exit.into(), free_vars }
            }
            Statement::Invoke(invoke) => {
                let arg_ids = invoke.args.bindings.iter().map(|a| a.var.id);
                let mut free_vars = HashSet::from_iter(arg_ids);
                free_vars.insert(invoke.var.id);
                InlineResult { result: invoke.into(), free_vars }
            }
        }
    }
}

impl Inline for Rc<Statement> {
    fn count_occurences(&self, defs: &HashMap<String, Def>, occurrences: &mut HashMap<String, Mark>) {
        self.as_ref().count_occurences(defs, occurrences);
    }

    fn inline(self, retain: &mut Vec<Def>, to_visit: &mut HashMap<String, Def>, occurrences: &HashMap<String, Mark>) -> InlineResult {
        Rc::unwrap_or_clone(self).inline(retain, to_visit, occurrences)
    }
}