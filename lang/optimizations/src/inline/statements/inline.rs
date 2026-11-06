use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use axcut::{
    syntax::{Def, Identifier, Statement},
    traits::free_vars::FreeVars,
};

use crate::inline::{Inline, InlineResult, Mark};

impl Inline for Statement {
    fn inline(
        self,
        retain: &mut Vec<Def>,
        to_visit: &mut HashMap<Identifier, Def>,
        occurrences: &HashMap<Identifier, Mark>,
    ) -> InlineResult {
        match self {
            Statement::Substitute(_) => {
                panic!("Inlining should be done before linearization")
            }
            Statement::Call(call) => call.inline(retain, to_visit, occurrences),
            Statement::Switch(mut switch) => {
                let mut all_free_vars = HashSet::new();
                switch.clauses = std::mem::take(&mut switch.clauses)
                    .into_iter()
                    .map(|mut c| {
                        let InlineResult {
                            result,
                            mut free_vars,
                        } = c.body.inline(retain, to_visit, occurrences);
                        for bind in c.context.bindings.iter() {
                            free_vars.remove(&bind.var.id);
                        }
                        all_free_vars.extend(free_vars);
                        c.body = result.into();
                        c
                    })
                    .collect();
                all_free_vars.insert(switch.var.id);
                InlineResult {
                    result: switch.into(),
                    free_vars: all_free_vars,
                }
            }
            Statement::Create(mut create) => {
                let InlineResult { result, free_vars } =
                    create.next.inline(retain, to_visit, occurrences);
                if !free_vars.contains(&create.var.id) {
                    InlineResult { result, free_vars }
                } else {
                    let mut all_free_vars = free_vars;
                    create.next = result.into();
                    create.clauses = std::mem::take(&mut create.clauses)
                        .into_iter()
                        .map(|mut c| {
                            let InlineResult {
                                result,
                                mut free_vars,
                            } = c.body.inline(retain, to_visit, occurrences);
                            for bind in c.context.bindings.iter() {
                                free_vars.remove(&bind.var.id);
                            }
                            all_free_vars.extend(free_vars);
                            c.body = result.into();
                            c
                        })
                        .collect();
                    all_free_vars.remove(&create.var.id);
                    InlineResult {
                        result: create.into(),
                        free_vars: all_free_vars,
                    }
                }
            }
            Statement::IfC(mut ifc) => {
                let mut all_free_vars = HashSet::new();
                let InlineResult { result, free_vars } =
                    ifc.thenc.inline(retain, to_visit, occurrences);
                all_free_vars.extend(free_vars);
                ifc.thenc = result.into();
                let InlineResult { result, free_vars } =
                    ifc.elsec.inline(retain, to_visit, occurrences);
                all_free_vars.extend(free_vars);
                ifc.elsec = result.into();
                all_free_vars.insert(ifc.fst.id);
                ifc.snd.as_ref().inspect(|n| {
                    all_free_vars.insert(n.id);
                });
                InlineResult {
                    result: ifc.into(),
                    free_vars: all_free_vars,
                }
            }
            Statement::Let(mut l) => {
                let InlineResult {
                    result,
                    mut free_vars,
                } = l.next.inline(retain, to_visit, occurrences);
                if !free_vars.contains(&l.var.id) {
                    InlineResult { result, free_vars }
                } else {
                    l.next = result.into();
                    free_vars.extend(l.args.bindings.iter().map(|b| b.var.id));
                    free_vars.remove(&l.var.id);
                    InlineResult {
                        result: l.into(),
                        free_vars,
                    }
                }
            }
            Statement::Literal(mut lit) => {
                let InlineResult {
                    result,
                    mut free_vars,
                } = lit.next.inline(retain, to_visit, occurrences);
                if !free_vars.contains(&lit.var.id) {
                    InlineResult { result, free_vars }
                } else {
                    lit.next = result.into();
                    free_vars.remove(&lit.var.id);
                    InlineResult {
                        result: lit.into(),
                        free_vars,
                    }
                }
            }
            Statement::Op(mut op) => {
                let InlineResult {
                    result,
                    mut free_vars,
                } = op.next.inline(retain, to_visit, occurrences);
                if !free_vars.contains(&op.var.id) {
                    InlineResult { result, free_vars }
                } else {
                    op.next = result.into();
                    free_vars.insert(op.fst.id);
                    free_vars.insert(op.snd.id);
                    free_vars.remove(&op.var.id);
                    InlineResult {
                        result: op.into(),
                        free_vars,
                    }
                }
            }
            Statement::PrintI64(mut print) => {
                let InlineResult {
                    result,
                    mut free_vars,
                } = print.next.inline(retain, to_visit, occurrences);
                print.next = result.into();
                free_vars.insert(print.var.id);
                InlineResult {
                    result: print.into(),
                    free_vars,
                }
            }
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
