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

type Xtor = Identifier;

pub struct GlobalSwitch {
    pub scrutinee_pos: usize,
    pub param_ids: Vec<ID>,
    pub clauses: Vec<Clause>
}

pub struct EliminateState<'a> {
    defs: Vec<Def>,
    global_switches: HashMap<Identifier, GlobalSwitch>,
    pub let_bindings: HashMap<Identifier, (Xtor, TypingContext)>,
    pub create_bindings: HashMap<Identifier, Vec<Clause>>,
    max_id: &'a mut usize //TODO: no ref, copy and then update at the end of elim
}

impl<'a> EliminateState<'a> {
    pub fn remove_defs(self) -> Vec<Def> {
        self.defs
    }

    pub fn readd_global_switch(&mut self, id: Identifier, info: GlobalSwitch) {
        self.global_switches.insert(id, info);
    }

    pub fn remove_global_switch(&mut self, id: &Identifier) -> Option<GlobalSwitch> {
        self.global_switches.remove(id)
    } 

    pub fn lift_global_switches(&mut self) {
        let mut defs = std::mem::take(&mut self.defs);
        let mut lifted_clauses = Vec::new();
        for def in defs.iter_mut() {
            if let Switch(switch) = &mut def.body {
                let lifted = self.lift_global_switch(switch, &def.name, &def.context);
                lifted_clauses.extend(lifted);
            }
        }
        defs.extend(lifted_clauses);
        self.defs = defs;
    }


    fn lift_global_switch(&mut self, switch: &mut SwitchStmt, name: &Identifier, context: &TypingContext) -> Vec<Def> {
        let mut lifted = Vec::new();
        switch.clauses = self.lift_clauses(std::mem::take(&mut switch.clauses), &mut lifted, true);
        let param_ids: Vec<usize> = context.bindings.iter().map(|b| b.var.id).collect();
        let scrutinee_pos = param_ids.iter().position(|id| *id == switch.var.id).unwrap();
        let info = GlobalSwitch { scrutinee_pos, param_ids, clauses: switch.clauses.clone() }; //TODO avoid clone?
        self.global_switches.insert(name.clone(), info);
        lifted
    }

    //TODO: remove global param?
    pub fn lift_clauses(&mut self, clauses: Vec<Clause>, lifted: &mut Vec<Def>, global: bool) -> Vec<Clause> {
        clauses.into_iter().map(|clause| self.lift_clause(clause, lifted, global)).collect()
    }
 
    fn lift_clause(&mut self, mut clause: Clause, lifted: &mut Vec<Def>, global: bool) -> Clause {
        match clause.body.as_ref() {
            Call(_) | Exit(_) | Invoke(_) => clause,
            body => {
                let fresh = fresh_identifier(self.max_id, "clause");
                let mut required_context = BTreeSet::new();
                body.typed_free_vars(&mut required_context);
                let old_param_ids: Vec<usize> = required_context.iter().map(|p| p.var.id).collect();
                let uniquified_params: Vec<ContextBinding> =
                    required_context.iter().map(
                        |p| { let mut uniq = p.clone(); uniq.var = fresh_identifier(self.max_id, &uniq.var.name); uniq }
                    ).collect();
                let renamings: Vec<(usize, Identifier)> =
                    old_param_ids.iter().zip(&uniquified_params).map(
                        |(id, uniq)| (*id, uniq.var.clone())
                    ).collect(); 
                let subst_body = body.clone().subst_sim(&renamings); //TODO
                let context = uniquified_params.into();

                if global && let Switch(mut switch) = subst_body.clone() { //TODO
                    let lifted_clauses = self.lift_global_switch(&mut switch, &fresh, &context);
                    lifted.extend(lifted_clauses);
                }
                let def = Def {
                    name: fresh.clone(),
                    context,
                    body: subst_body
                };
                lifted.push(def);

                let call = Call(CallStmt { 
                    label: fresh,
                    args: TypingContext { bindings: required_context.into_iter().collect() }
                });
                clause.body = call.into();
                clause
            }
        }
    }
}

pub trait Eliminate {
    type Target;
    fn eliminate(self, context: &mut EliminateState) -> Self::Target;
}

impl<T: Eliminate + Clone> Eliminate for Rc<T> {
    type Target = Rc<T::Target>;
    fn eliminate(self, context: &mut EliminateState) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).eliminate(context))
    }
}
