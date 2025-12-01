use axcut::syntax::{
    ContextBinding, Def, Name, TypingContext, Var, names::fresh_name, statements::Clause,
};
use axcut::traits::typed_free_vars::TypedFreeVars;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    rc::Rc,
};

/// State during rewriting
pub struct RewriteState {
    /// Names of definitions ihn the current program
    pub used_labels: HashSet<Name>,
    /// Definitions in the current program
    pub lifted_statements: Vec<Def>,
    /// used vars in the current definition
    pub current_used_vars: HashSet<Var>,
    /// Name of the current definition
    pub current_label: String,
    /// Let bindings defined in the current definition
    /// Keys are the bound variables
    /// values are xtor names and arguments
    pub let_bindings: HashMap<Var, (Name, TypingContext)>,
    /// Create bindings used in the current definition
    /// keys are bound variables
    /// values are create clauses
    pub create_bindings: HashMap<Var, Vec<Clause>>,
    /// Has anything been changed during the current pass
    pub new_changes: bool,
}

impl RewriteState {
    /// Updates the current definition
    pub fn set_current_def(&mut self, def_name: &str, def_vars: HashSet<String>) {
        self.current_label = def_name.to_owned();
        self.current_used_vars = def_vars;
        self.let_bindings.clear();
        self.create_bindings.clear();
        self.new_changes = false;
    }

    pub fn add_def(&mut self, def: Def) {
        match self
            .lifted_statements
            .iter()
            .enumerate()
            .find(|(_, df)| *df.name == def.name)
        {
            None => self.lifted_statements.push(def),
            Some((ind, _)) => self.lifted_statements[ind] = def,
        }
    }

    pub fn get_let(&self, var: &Var) -> Option<(Name, TypingContext)> {
        self.let_bindings.get(var).cloned()
    }

    pub fn get_create_clause(&self, var: &Var, xtor: &Name) -> Option<(Clause, usize)> {
        self.create_bindings.get(var).map(|clauses| {
            let position = clauses
                .iter()
                .position(|clause| clause.xtor == *xtor)
                .unwrap_or_else(|| panic!("Could not find create clause binding for {xtor}"));
            (clauses[position].clone(), position)
        })
    }

    pub fn lift_create_clause(
        &mut self,
        mut clause: Clause,
        bound_var: &Var,
    ) -> (String, BTreeSet<ContextBinding>) {
        let name = fresh_name(
            &mut self.used_labels,
            &("lift_".to_string() + &self.current_label + "_" + bound_var + "_" + &clause.xtor),
        );
        let mut free_vars = BTreeSet::new();
        clause.typed_free_vars(&mut free_vars);
        clause.context.bindings.extend(free_vars.clone());

        let def = Def {
            name: name.clone(),
            context: clause.context,
            used_vars: self.current_used_vars.clone(),
            body: Rc::unwrap_or_clone(clause.body),
        };
        self.lifted_statements.push(def);
        (name, free_vars)
    }

    pub fn lift_switch_call(
        &mut self,
        switch_def: &Name,
        switch_var: &Var,
        def_args: &TypingContext,
        clause: &Clause,
    ) -> (String, Vec<ContextBinding>) {
        let name = fresh_name(
            &mut self.used_labels,
            &("lift_".to_string() + switch_def + "_" + switch_var + "_" + &clause.xtor),
        );
        self.used_labels.insert(name.clone());

        let mut new_context = clause.context.clone();
        new_context.bindings.extend(
            def_args
                .bindings
                .iter()
                .filter(|bind| bind.var != *switch_var)
                .cloned(),
        );
        let new_def = Def {
            name: name.clone(),
            context: new_context.clone(),
            body: Rc::unwrap_or_clone(clause.body.clone()),
            used_vars: HashSet::new(),
        };
        self.add_def(new_def);
        (name, new_context.bindings)
    }

    pub fn lift_create_call(&mut self, create_def: &Name, create_var: &Var, clause: Clause) {
        let name = fresh_name(
            &mut self.used_labels,
            &("lift_".to_string() + create_def + "_" + create_var + "_" + &clause.xtor),
        );
        let mut used_vars = BTreeSet::new();
        clause.body.typed_free_vars(&mut used_vars);
        self.used_labels.insert(name.clone());
        let new_def = Def {
            name,
            used_vars: used_vars.into_iter().map(|bnd| bnd.var).collect(),
            context: clause.context,
            body: Rc::unwrap_or_clone(clause.body),
        };
        self.add_def(new_def);
    }
}

pub trait Rewrite {
    type Target;
    fn rewrite(self, context: &mut RewriteState) -> Self::Target;
}

impl<T: Rewrite + Clone> Rewrite for Rc<T> {
    type Target = Rc<T::Target>;
    fn rewrite(self, context: &mut RewriteState) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).rewrite(context))
    }
}
