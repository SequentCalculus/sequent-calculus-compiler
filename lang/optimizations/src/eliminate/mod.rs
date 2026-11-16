pub mod program;
pub mod statements;

use axcut::syntax::Statement::{Call, Exit, Invoke, Switch};
use axcut::syntax::names::fresh_identifier;
use axcut::syntax::statements::{Call as CallStmt, Clause, Switch as SwitchStmt};
use axcut::syntax::{
    ContextBinding, Def, ID, Identifier, TypingContext,
};
use axcut::traits::substitution::Subst;
use axcut::traits::typed_free_vars::TypedFreeVars;
use std::collections::BTreeSet;
use std::{
    collections::HashMap,
    rc::Rc,
};

pub trait Eliminate {
    type Target;
    fn eliminate(self, state: &mut EliminateState, lifted: &mut Vec<Def>) -> Self::Target;
}

impl<T: Eliminate + Clone> Eliminate for Rc<T> {
    type Target = Rc<T::Target>;
    fn eliminate(self, state: &mut EliminateState, lifted: &mut Vec<Def>) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).eliminate(state, lifted))
    }
}

type Xtor = Identifier;

pub struct GlobalSwitch {
    pub scrutinee_pos: usize,
    pub param_ids: Vec<ID>,
    pub clauses: Vec<Clause>
}

pub struct EliminateState {
    global_switches: HashMap<Identifier, GlobalSwitch>,
    pub let_bindings: HashMap<Identifier, (Xtor, TypingContext)>,
    pub create_bindings: HashMap<Identifier, Vec<Clause>>,
    max_id: usize
}

impl EliminateState {
    pub fn readd_global_switch(&mut self, id: Identifier, info: GlobalSwitch) {
        self.global_switches.insert(id, info);
    }

    pub fn remove_global_switch(&mut self, id: &Identifier) -> Option<GlobalSwitch> {
        self.global_switches.remove(id)
    } 

    pub fn lift_global_switches(&mut self, defs: &mut Vec<Def>) {
        let mut lifted_clauses = Vec::new();
        for def in defs.iter_mut() {
            if let Switch(switch) = &mut def.body {
                let lifted = self.lift_global_switch(switch, &def.name, &def.context);
                lifted_clauses.extend(lifted);
            }
        }
        defs.extend(lifted_clauses);
    }


    fn lift_global_switch(&mut self, switch: &mut SwitchStmt, name: &Identifier, context: &TypingContext) -> Vec<Def> {
        let mut lifted = Vec::new();
        switch.clauses = self.lift_clauses(std::mem::take(&mut switch.clauses), &mut lifted, true);
        let param_ids: Vec<usize> = context.bindings.iter().map(|b| b.var.id).collect();
        let scrutinee_pos = param_ids.iter().position(|id| *id == switch.var.id).unwrap();
        let info = GlobalSwitch { scrutinee_pos, param_ids, clauses: switch.clauses.clone() };
        self.global_switches.insert(name.clone(), info);
        lifted
    }

    pub fn lift_clauses(&mut self, clauses: Vec<Clause>, lifted: &mut Vec<Def>, global: bool) -> Vec<Clause> {
        clauses.into_iter().map(|clause| self.lift_clause(clause, lifted, global)).collect()
    }
 
    fn lift_clause(&mut self, mut clause: Clause, lifted: &mut Vec<Def>, global: bool) -> Clause {
        let body = Rc::unwrap_or_clone(clause.body);
        match body {
            Call(_) | Exit(_) | Invoke(_) => {
                clause.body = Rc::new(body);
                clause
            },
            body => {
                let lifted_ident = fresh_identifier(&mut self.max_id, "lifted_clause");
                let mut required_context = BTreeSet::new();
                body.typed_free_vars(&mut required_context);
                let old_ids: Vec<usize> = required_context.iter().map(|bind| bind.var.id).collect();
                let uniquified_params: Vec<ContextBinding> =
                    required_context.iter().map(|bind| {
                        let mut uniq = bind.clone();
                        uniq.var = fresh_identifier(&mut self.max_id, &uniq.var.name);
                        uniq 
                    }).collect();
                let subst: Vec<(usize, Identifier)> =
                    old_ids.into_iter().zip(&uniquified_params).map(
                        |(id, uniq)| (id, uniq.var.clone())
                    ).collect();
                let mut subst_body = body.subst_sim(&subst);
                let lifted_context = uniquified_params.into();

                if global && let Switch(switch) = &mut subst_body {
                    let lifted_clauses = self.lift_global_switch(switch, &lifted_ident, &lifted_context);
                    lifted.extend(lifted_clauses);
                }

                let def = Def {
                    name: lifted_ident.clone(),
                    context: lifted_context,
                    body: subst_body
                };
                lifted.push(def);

                let call = Call(CallStmt { 
                    label: lifted_ident,
                    args: TypingContext { bindings: required_context.into_iter().collect() }
                });
                clause.body = call.into();
                clause
            }
        }
    }
}
