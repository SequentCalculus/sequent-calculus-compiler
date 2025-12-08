use axcut::syntax::{
    ContextBinding, Def, Name, TypingContext, Var,
    names::fresh_name,
    statements::{Call, Clause},
};
use axcut::traits::typed_free_vars::TypedFreeVars;
use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    rc::Rc,
};

pub struct RewriteState<'a> {
    pub used_vars: &'a HashSet<Var>,
    pub used_labels: &'a mut HashSet<Name>,
    pub current_label: &'a str,
    pub lifted_statements: &'a mut VecDeque<Def>,
    pub let_bindings: HashMap<Var, (Name, TypingContext)>,
    pub create_bindings: HashMap<Var, Vec<Clause>>,
    pub new_changes: &'a mut bool,
}

impl RewriteState<'_> {
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

    pub fn lift_clause(
        &mut self,
        mut clause: Clause,
        bound_var: &Var,
    ) -> (String, BTreeSet<ContextBinding>) {
        let name = fresh_name(
            self.used_labels,
            &("lift_".to_string() + self.current_label + "_" + bound_var + "_" + &clause.xtor),
        );
        let mut free_vars = BTreeSet::new();
        clause.typed_free_vars(&mut free_vars);
        clause.context.bindings.extend(free_vars.clone());

        let def = Def {
            name: name.clone(),
            context: clause.context,
            used_vars: self.used_vars.clone(),
            body: Rc::unwrap_or_clone(clause.body),
        };
        self.lifted_statements.push_back(def);

        (name, free_vars)
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
