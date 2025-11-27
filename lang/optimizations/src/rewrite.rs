use crate::errors::Error;
use axcut::syntax::{
    ContextBinding, Def, Name, TypingContext, Var, fresh_name, statements::Clause,
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

    pub fn get_create_clause(
        &self,
        var: &Var,
        tag: &Name,
    ) -> Option<Result<(Clause, usize), Error>> {
        self.create_bindings.get(var).map(|clauses| {
            let clause_err = Error::create_clause(clauses, tag);
            let position = clauses
                .iter()
                .position(|clause| clause.xtor == *tag)
                .ok_or(clause_err)?;
            Ok((clauses[position].clone(), position))
        })
    }

    pub fn lift_clause(
        &mut self,
        mut clause: Clause,
        bound_var: &Var,
    ) -> Result<(String, BTreeSet<ContextBinding>), Error> {
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
        self.lifted_statements.push_front(def);

        Ok((name, free_vars))
    }
}

pub trait Rewrite {
    type Target;
    fn rewrite(self, context: &mut RewriteState) -> Result<Self::Target, Error>;
}

impl<T: Rewrite + Clone> Rewrite for Rc<T> {
    type Target = Rc<T::Target>;
    fn rewrite(self, context: &mut RewriteState) -> Result<Self::Target, Error> {
        Ok(Rc::new(Rc::unwrap_or_clone(self).rewrite(context)?))
    }
}
