use crate::{Error, Rewrite, free_bindings::FreeBindings};
use axcut::syntax::{
    ContextBinding, Def, Name, TypingContext, Var,
    statements::{Clause, Create, Let},
};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub struct RewriteContext {
    pub current_def: Name,
    pub current_used_vars: HashSet<Var>,
    pub current_def_runs: u64,
    pub rewritten_defs: HashMap<Name, Def>,
    let_bindings: HashMap<Var, (Name, TypingContext)>,
    create_bindings: HashMap<Var, Vec<Clause>>,
    pub new_changes: bool,
}

impl RewriteContext {
    pub fn new() -> Self {
        Self {
            current_def: String::new(),
            current_used_vars: HashSet::new(),
            rewritten_defs: HashMap::new(),
            let_bindings: HashMap::new(),
            create_bindings: HashMap::new(),
            new_changes: false,
            current_def_runs: 0,
        }
    }

    pub fn set_def(&mut self, def_name: &str, def_vars: &HashSet<String>) {
        self.current_def = def_name.to_owned();
        self.current_used_vars = def_vars.clone();
        self.let_bindings.clear();
        self.create_bindings.clear();
        self.new_changes = false;
    }

    pub fn add_def(&mut self, def: Def) {
        self.rewritten_defs.insert(def.name.clone(), def);
    }

    pub fn add_let(&mut self, lt: &Let) {
        self.let_bindings
            .insert(lt.var.clone(), (lt.tag.clone(), lt.args.clone()));
    }

    pub fn add_create(&mut self, create: &Create) {
        self.create_bindings
            .insert(create.var.clone(), create.clauses.clone());
    }

    pub fn get_let(&self, var: &Var) -> Option<(Name, TypingContext)> {
        self.let_bindings.get(var).cloned()
    }

    pub fn get_create(&self, var: &Var) -> Option<Vec<Clause>> {
        self.create_bindings.get(var).cloned()
    }

    pub fn lifted_name(&self, clause_xtor: &Name, bound_var: &Var) -> String {
        format!("{}_{}_{}_lifted", self.current_def, bound_var, clause_xtor)
    }

    pub fn already_lifted(&self, def_name: &Name) -> bool {
        self.rewritten_defs.contains_key(def_name)
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
        self.rewritten_defs.insert(new_name, new_def);
        Ok(())
    }
}
