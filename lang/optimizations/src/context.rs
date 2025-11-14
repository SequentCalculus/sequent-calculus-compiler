use crate::{Error, GetUsedVars, Rewrite};
use axcut::syntax::{
    Def, Name, Var,
    statements::{Clause, Create, Let},
};
use std::{collections::HashMap, rc::Rc};

pub struct RewriteContext {
    current_def: String,
    let_bindings: HashMap<Var, Let>,
    create_bindings: HashMap<Var, Create>,
    pub lifted_defs: HashMap<Name, Def>,
    pub new_changes: bool,
}

impl RewriteContext {
    pub fn new(current_def: &str) -> Self {
        Self {
            current_def: current_def.to_owned(),
            let_bindings: HashMap::new(),
            create_bindings: HashMap::new(),
            new_changes: false,
            lifted_defs: HashMap::new(),
        }
    }

    pub fn add_let(&mut self, lt: &Let) {
        self.let_bindings.insert(lt.var.clone(), lt.clone());
    }

    pub fn add_create(&mut self, create: &Create) {
        self.create_bindings
            .insert(create.var.clone(), create.clone());
    }

    pub fn get_let(&self, var: &Var) -> Option<Let> {
        self.let_bindings.get(var).cloned()
    }

    pub fn get_create(&self, var: &Var) -> Option<Create> {
        self.create_bindings.get(var).cloned()
    }

    pub fn lifted_name(&self, clause_xtor: &Name) -> String {
        format!("{}_{}_lifted", self.current_def, clause_xtor)
    }

    pub fn already_lifted(&self, def_name: &Name) -> bool {
        self.lifted_defs.contains_key(def_name)
    }

    pub fn lift_clause(&mut self, clause: Clause) -> Result<(), Error> {
        let new_name = self.lifted_name(&clause.xtor);
        let new_body = Rc::unwrap_or_clone(clause.body.rewrite(self)?);
        let new_def = Def {
            name: new_name.clone(),
            context: clause.context,
            used_vars: new_body.get_used_vars(),
            body: new_body,
        };
        self.lifted_defs.insert(new_name, new_def);
        Ok(())
    }
}
