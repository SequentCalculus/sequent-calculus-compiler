use axcut::{syntax::{Def, ID, Identifier, Prog, Statement}, traits::substitution::Subst};
use std::{cell::Cell, collections::{HashMap, VecDeque}, rc::Rc};

use crate::inline::{Inline, Mark, Occurrences};

// TODO: check if make_mut clones here
// TODO: cycles :(
fn inline_bfs(def_map: &mut HashMap<Identifier, DefInfo>, mut queue: VecDeque<(&mut Statement, Rc<Cell<i32>>)>) {
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
                budget.replace(budget.get() - 1);
                for clause in create.clauses.iter_mut() {
                    let body: &mut Statement = Rc::make_mut(&mut clause.body);
                    queue.push_back((body, Rc::clone(&budget)));
                }
                let next: &mut Statement = Rc::make_mut(&mut create.next);
                queue.push_back((next, budget));
            },
            Statement::Switch(switch) => {
                budget.replace(budget.get() - 1);
                for clause in switch.clauses.iter_mut() {
                    let body: &mut Statement = Rc::make_mut(&mut clause.body);
                    queue.push_back((body, Rc::clone(&budget)));
                }
            }
            Statement::IfC(ifc) => {
                budget.replace(budget.get() - 1);
                let thn: &mut Statement = Rc::make_mut(&mut ifc.thenc);
                let els: &mut Statement = Rc::make_mut(&mut ifc.elsec);
                queue.push_back((thn, Rc::clone(&budget)));
                queue.push_back((els, Rc::clone(&budget)));
            }
            Statement::Substitute(subst) => {
                budget.replace(budget.get() - 1);
                let next: &mut Statement = Rc::make_mut(&mut subst.next);
                queue.push_back((next, budget));
            }
            Statement::Let(l) => {
                budget.replace(budget.get() - 1);
                let next: &mut Statement = Rc::make_mut(&mut l.next);
                queue.push_back((next, budget));
            }
            Statement::Literal(lit) => {
                budget.replace(budget.get() - 1);
                let next: &mut Statement = Rc::make_mut(&mut lit.next);
                queue.push_back((next, budget));
            }
            Statement::Op(op) => {
                budget.replace(budget.get() - 1);
                let next: &mut Statement = Rc::make_mut(&mut op.next);
                queue.push_back((next, budget));
            }
            Statement::PrintI64(print) => {
                budget.replace(budget.get() - 1);
                let next: &mut Statement = Rc::make_mut(&mut print.next);
                queue.push_back((next, budget));
            }
            Statement::Invoke(_) | Statement::Exit(_) => {}
        }
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

/// Inlines all definitions with 0 or 1 textual occurrences
/// Inlines (/removes) all bindings that are not used
pub fn inline(prog: &mut Prog) {
    let original_order: HashMap<Identifier, usize> = prog
        .defs
        .iter()
        .enumerate()
        .map(|(i, d)| (d.name.clone(), i))
        .collect();

    let defs = std::mem::take(&mut prog.defs);
    let mut def_map: HashMap<Identifier, DefInfo> = HashMap::with_capacity(defs.len());
    def_map.extend(defs.into_iter().map(|def| {
        let len = def.body.len();
        (def.name.clone(), DefInfo { def, len })
    }));

    let mut defs_for_queue: Vec<Def> = def_map.values().map(|di| &di.def).cloned().collect();
    let default_budget = 1000;
    let queue: VecDeque<(&mut Statement, Rc<Cell<i32>>)> =
        defs_for_queue.iter_mut().map(|d| (&mut d.body, Rc::new(Cell::new(default_budget)))).collect();

    inline_bfs(&mut def_map, queue);

    prog.defs.extend(defs_for_queue);
    prog.defs.sort_unstable_by_key(|d| original_order[&d.name]);
}
