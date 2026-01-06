use axcut::syntax::{
    ContextBinding, Def, Name, TypingContext, Var,
    names::fresh_name,
    statements::{Call, Clause, Statement, Switch},
};
use axcut::traits::typed_free_vars::TypedFreeVars;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    rc::Rc,
};

pub struct SwitchInfo {
    pub switch: Switch,
    pub clause_position: usize,
    pub called_def_position: usize,
    pub let_args: TypingContext,
}

/// State during rewriting
pub struct RewriteState {
    /// `Name`s of definitions in the current program
    pub used_labels: HashSet<Name>,
    /// `Def`initions in the current program
    pub defs: Vec<Def>,
    /// Used `Var`s in the current definition
    pub current_used_vars: HashSet<Var>,
    /// Name of the current definition
    pub current_label: String,
    /// `Let` bindings defined in the current definition: keys are the bound variables, values are
    /// the correspinding xtor names and arguments
    pub let_bindings: HashMap<Var, (Name, TypingContext)>,
    /// `Create` bindings defined in the current definition: keys are the bound variables, values
    /// are the correspinding clauses
    pub create_bindings: HashMap<Var, Vec<Clause>>,
    /// Tracks whether there have been changes during the current pass
    pub new_changes: bool,
}

impl RewriteState {
    pub fn get_let(&self, var: &Var) -> Option<(Name, TypingContext)> {
        self.let_bindings.get(var).cloned()
    }

    pub fn get_create_clause(&self, var: &Var, xtor: &Name) -> Option<(Clause, usize)> {
        let clauses = self.create_bindings.get(var)?;
        let position = clauses
            .iter()
            .position(|clause| clause.xtor == *xtor)
            .unwrap_or_else(|| panic!("Could not find create clause for {xtor}"));
        Some((clauses[position].clone(), position))
    }

    pub fn get_switch_info(
        &mut self,
        called_label: &Name,
        called_args: &TypingContext,
    ) -> Option<SwitchInfo> {
        let called_def_position = self
            .defs
            .iter()
            .position(|def| def.name == *called_label)
            .unwrap_or_else(|| panic!("Could not find label {called_label}"));
        let called_def = &mut self.defs[called_def_position];
        let switch_var = match &called_def.body {
            Statement::Switch(switch) => &switch.var,
            _ => return None,
        };

        let switch_var_position = called_def
            .context
            .bindings
            .iter()
            .position(|binding| binding.var == *switch_var)
            .unwrap_or_else(|| panic!("Could not find argument of Switch variable {switch_var}"));
        let (tag, let_args) = self
            .let_bindings
            .get(&called_args.bindings[switch_var_position].var)?;

        let Statement::Switch(switch) = std::mem::take(&mut called_def.body) else {
            panic!("Statement must be a Switch")
        };
        let clause_position = switch
            .clauses
            .iter()
            .position(|clause| clause.xtor == *tag)
            .unwrap_or_else(|| panic!("Could not find Switch Clause for {tag}"));

        Some(SwitchInfo {
            switch,
            clause_position,
            called_def_position,
            let_args: let_args.clone(),
        })
    }

    pub fn lift_create_clause(
        &mut self,
        clause: Clause,
        position: usize,
        bound_var: &Var,
    ) -> (Name, Vec<usize>, Vec<ContextBinding>) {
        let name = fresh_name(
            &mut self.used_labels,
            &("lift_".to_string() + &self.current_label + "_" + bound_var + "_" + &clause.xtor),
        );

        let mut free_vars = BTreeSet::new();
        clause.body.typed_free_vars(&mut free_vars);
        let (arg_positions, mut clause_bindings): (Vec<_>, Vec<_>) = clause
            .context
            .bindings
            .into_iter()
            .enumerate()
            .filter(|(_, binding)| free_vars.contains(binding))
            .unzip();
        for binding in &clause_bindings {
            free_vars.remove(binding);
        }
        let free_vars: Vec<_> = free_vars.into_iter().collect();
        let mut context = free_vars.clone();
        context.append(&mut clause_bindings);

        // we have to rewrite the Create whose Clause we lift to avoid duplication
        let create = self
            .create_bindings
            .get_mut(bound_var)
            .unwrap_or_else(|| panic!("Could not find create for variable {bound_var}"));
        create[position].body = Rc::new(
            Call {
                label: name.clone(),
                args: context.clone().into(),
            }
            .into(),
        );

        let def = Def {
            name: name.clone(),
            context: context.into(),
            used_vars: self.current_used_vars.clone(),
            body: Rc::unwrap_or_clone(clause.body),
        };
        self.defs.push(def);

        (name, arg_positions, free_vars)
    }

    pub fn lift_switch_clause(
        &mut self,
        switch_info: &mut SwitchInfo,
        label: &Name,
    ) -> (Name, Vec<usize>) {
        let clause = &mut switch_info.switch.clauses[switch_info.clause_position];
        let called_def = &self.defs[switch_info.called_def_position];

        let name = fresh_name(
            &mut self.used_labels,
            &("lift_".to_string() + label + "_" + &switch_info.switch.var + "_" + &clause.xtor),
        );

        let mut free_vars = BTreeSet::new();
        clause.body.typed_free_vars(&mut free_vars);
        let (arg_positions, bindings): (Vec<_>, Vec<_>) = called_def
            .context
            .bindings
            .iter()
            .chain(clause.context.bindings.iter())
            .enumerate()
            .filter(|(_, binding)| free_vars.contains(binding))
            .map(|(position, binding)| (position, binding.clone()))
            .unzip();

        // we have to rewrite the Switch whose Clause we lift to avoid duplication
        let body = std::mem::replace(
            &mut clause.body,
            Rc::new(
                Call {
                    label: name.clone(),
                    args: bindings.clone().into(),
                }
                .into(),
            ),
        );

        let def = Def {
            name: name.clone(),
            context: bindings.into(),
            used_vars: called_def.used_vars.clone(),
            body: Rc::unwrap_or_clone(body),
        };
        self.defs.push(def);

        (name, arg_positions)
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
