use crate::{Error, Rewrite};
use axcut::{
    syntax::{
        ContextBinding, Def, Name, TypingContext, Var,
        statements::{Clause, Create, Let},
    },
    traits::typed_free_vars::TypedFreeVars,
};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    rc::Rc,
};

/// Context during rewriting a program
pub struct RewriteContext {
    /// All definitions in the current program
    /// either already rewritten or still to be rewritten
    pub definitions: Vec<Def>,
    /// Current definition being rewritten
    pub current_def: Name,
    /// Used vars of the current definition (corresponds do [`axcut::syntax::Def::used_vars`])
    pub current_used_vars: HashSet<Var>,
    /// number of iterations ran on the current definition
    pub current_def_runs: u64,
    /// let bindings defined in the current definition
    /// keys are the variables bound by [`axcut::syntax::statements::Let`]
    /// values are [`axcut::syntax::statements::Let::tag`] and
    /// [`axcut::syntax::statements::Let::args`]
    let_bindings: HashMap<Var, (Name, TypingContext)>,
    /// create bindings defined in the current definition
    /// keys are the variables bound by [`axcut::syntax::statements::Create`]
    /// values are the bound clauses
    create_bindings: HashMap<Var, Vec<Clause>>,
    /// were there new changes in the current run
    /// if not the current definition is not rewritten again
    pub new_changes: bool,
}

impl RewriteContext {
    pub fn new(defs: Vec<Def>) -> Self {
        Self {
            current_def: String::new(),
            current_used_vars: HashSet::new(),
            definitions: defs,
            let_bindings: HashMap::new(),
            create_bindings: HashMap::new(),
            new_changes: false,
            current_def_runs: 0,
        }
    }

    pub fn set_current_def(&mut self, def_name: &str, def_vars: &HashSet<String>) {
        self.current_def = def_name.to_owned();
        self.current_used_vars = def_vars.clone();
        self.let_bindings.clear();
        self.create_bindings.clear();
        self.new_changes = false;
    }

    pub fn add_def(&mut self, def: Def) {
        match self
            .definitions
            .iter()
            .enumerate()
            .find(|(_, df)| *df.name == def.name)
        {
            None => self.definitions.push(def),
            Some((ind, _)) => self.definitions[ind] = def,
        }
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

    pub fn get_def(&self, name: &Name) -> Option<Def> {
        self.definitions
            .iter()
            .find(|def| def.name == *name)
            .cloned()
    }

    pub fn create_lifted(&self, create_def: &Name, xtor: &Name, create_var: &Var) -> String {
        format!("{create_def}_create_{xtor}_{create_var}_lifted")
    }

    pub fn switch_lifted(&self, switch_def: &Name, xtor: &Name, switch_var: &Var) -> String {
        format!("{switch_def}_switch_{xtor}_{switch_var}_lifted")
    }

    pub fn already_lifted(&self, def_name: &Name) -> bool {
        self.definitions.iter().any(|def| def.name == *def_name)
    }

    pub fn lift_create_clause(&mut self, clause: Clause, bound_var: &Var) -> Result<(), Error> {
        let new_name = self.create_lifted(&self.current_def, &clause.xtor, bound_var);
        let mut next_bindings = BTreeSet::new();
        clause.typed_free_vars(&mut next_bindings);
        let mut next_bindings: Vec<ContextBinding> = next_bindings.into_iter().collect();
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
        self.definitions.push(new_def);
        Ok(())
    }

    pub fn lift_switch_call(
        &mut self,
        switch_def: &Name,
        switch_var: &Var,
        clause: &Clause,
        extra_args: Vec<ContextBinding>,
    ) {
        let lifted_name = self.switch_lifted(switch_def, &clause.xtor, switch_var);
        let mut new_context = clause.context.clone();
        new_context.bindings.extend(extra_args);
        let new_def = Def {
            name: lifted_name,
            context: new_context,
            body: Rc::unwrap_or_clone(clause.body.clone()),
            used_vars: HashSet::new(),
        };
        self.add_def(new_def);
    }

    pub fn lift_create_call(&mut self, create_def: &Name, create_var: &Var, clause: Clause) {
        let lifted_name = self.create_lifted(create_def, &clause.xtor, create_var);
        let mut used_vars = BTreeSet::new();
        clause.body.typed_free_vars(&mut used_vars);
        let new_def = Def {
            name: lifted_name,
            used_vars: used_vars.into_iter().map(|bnd| bnd.var).collect(),
            context: clause.context,
            body: Rc::unwrap_or_clone(clause.body),
        };
        self.add_def(new_def);
    }
}
