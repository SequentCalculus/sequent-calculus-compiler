use axcut::{syntax::{Def, ID, Identifier, Prog, Statement}, traits::substitution::Subst};
use std::{collections::{HashMap, VecDeque}, rc::Rc};

use super::{Budget, DefInfo, Length};

// TODO: check if make_mut clones here
// TODO: one statement def call cycles :(
// TODO: uniquify body when inlining, as unique idents are an invariant
fn inline_bfs(def_map: &mut HashMap<Identifier, DefInfo>, mut queue: VecDeque<(&mut Statement, Budget)>) {
    while let Some((stmt, budget)) = queue.pop_front() {
        match stmt {
            Statement::Call(call) => {
                let callee = def_map.get(&call.label).unwrap();
                if callee.len <= budget.get() {
                    let arg_iter = call.args.bindings.iter().map(|a| a.var.clone());
                    let param_id_iter = callee.def.context.bindings.iter().map(|p| p.var.id);
                    let subst: Vec<(ID, Identifier)> = param_id_iter.zip(arg_iter).collect();
                    *stmt = callee.def.body.clone().subst_sim(&subst);
                    queue.push_back((stmt, budget));
                }
            },
            Statement::Create(create) => {
                budget.decr();
                for clause in create.clauses.iter_mut() {
                    let body: &mut Statement = Rc::make_mut(&mut clause.body);
                    queue.push_back((body, budget.clone()));
                }
                let next: &mut Statement = Rc::make_mut(&mut create.next);
                queue.push_back((next, budget));
            },
            Statement::Switch(switch) => {
                budget.decr();
                for clause in switch.clauses.iter_mut() {
                    let body: &mut Statement = Rc::make_mut(&mut clause.body);
                    queue.push_back((body, budget.clone()));
                }
            }
            Statement::IfC(ifc) => {
                budget.decr();
                let thn: &mut Statement = Rc::make_mut(&mut ifc.thenc);
                let els: &mut Statement = Rc::make_mut(&mut ifc.elsec);
                queue.push_back((thn, budget.clone()));
                queue.push_back((els, budget.clone()));
            }
            Statement::Substitute(subst) => {
                budget.decr();
                let next: &mut Statement = Rc::make_mut(&mut subst.next);
                queue.push_back((next, budget));
            }
            Statement::Let(l) => {
                budget.decr();
                let next: &mut Statement = Rc::make_mut(&mut l.next);
                queue.push_back((next, budget));
            }
            Statement::Literal(lit) => {
                budget.decr();
                let next: &mut Statement = Rc::make_mut(&mut lit.next);
                queue.push_back((next, budget));
            }
            Statement::Op(op) => {
                budget.decr();
                let next: &mut Statement = Rc::make_mut(&mut op.next);
                queue.push_back((next, budget));
            }
            Statement::PrintI64(print) => {
                budget.decr();
                let next: &mut Statement = Rc::make_mut(&mut print.next);
                queue.push_back((next, budget));
            }
            Statement::Invoke(_) | Statement::Exit(_) => {}
        }
    }
}

/// Inlines into definitions up to a certain size limit
pub fn inline(prog: &mut Prog) {
    prog.use_defs(|defs| {
        let original_order: HashMap<Identifier, usize> =
            defs.iter().enumerate()
            .map(|(i, d)| (d.name.clone(), i))
            .collect();

        let mut def_map: HashMap<Identifier, DefInfo> = HashMap::with_capacity(defs.len());
        def_map.extend(defs.into_iter().map(|def| {
            let len = def.body.len();
            (def.name.clone(), DefInfo { def, len })
        }));

        let mut defs_for_queue: Vec<Def> = def_map.values().map(|di| &di.def).cloned().collect();
        let queue: VecDeque<(&mut Statement, Budget)> =
            defs_for_queue.iter_mut().map(|d| (&mut d.body, Budget::new(1000))).collect();

        inline_bfs(&mut def_map, queue);
        defs_for_queue.sort_unstable_by_key(|d| original_order[&d.name]);
        defs_for_queue
    });
}
