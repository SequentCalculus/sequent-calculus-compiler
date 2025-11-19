use crate::{Error, Rewrite, free_bindings::FreeBindings};
use axcut::syntax::{
    ContextBinding, Def, Name, Var,
    statements::{Clause, Create, Let},
};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub struct RewriteContext {
    pub current_def: String,
    pub current_used_vars: HashSet<Var>,
    let_bindings: HashMap<Var, Let>,
    create_bindings: HashMap<Var, Create>,
    pub lifted_defs: HashMap<Name, Def>,
    pub new_changes: bool,
}

impl RewriteContext {
    pub fn new(current_def: &str, current_vars: &HashSet<Var>) -> Self {
        Self {
            current_def: current_def.to_owned(),
            current_used_vars: current_vars.clone(),
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

    pub fn lifted_name(&self, clause_xtor: &Name, bound_var: &Var) -> String {
        format!("{}_{}_{}_lifted", self.current_def, bound_var, clause_xtor)
    }

    pub fn already_lifted(&self, def_name: &Name) -> bool {
        self.lifted_defs.contains_key(def_name)
    }

    pub fn lift_clause(&mut self, clause: Clause, bound_var: &Var) -> Result<(), Error> {
        let new_name = self.lifted_name(&clause.xtor, bound_var);
        let mut next_bindings: Vec<ContextBinding> = clause.free_bindings().into_iter().collect();
        next_bindings.sort();
        let mut new_context = clause.context;
        new_context.bindings.extend(next_bindings);
        let new_body = Rc::unwrap_or_clone(clause.body.rewrite(self)?);

        let new_def = Def {
            name: new_name.clone(),
            context: new_context,
            used_vars: self.current_used_vars.clone(),
            body: new_body,
        };
        self.lifted_defs.insert(new_name, new_def);
        Ok(())
    }
}
