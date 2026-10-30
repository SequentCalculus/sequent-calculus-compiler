use axcut::syntax::{
    ContextBinding, Def, TypingContext,
    names::{ID, Identifier, fresh_identifier},
    statements::{Call, Clause, Statement, Switch},
};
use axcut::traits::{substitution::Subst, typed_free_vars::TypedFreeVars};
use printer::Print;

use std::{
    collections::{BTreeSet, HashMap},
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
    /// `Def`initions in the current program
    pub defs: Vec<Def>,
    /// Name of the current definition
    pub current_label: Identifier,
    /// `Let` bindings defined in the current definition: keys are the bound variables, values are
    /// the correspinding xtor names and arguments
    pub let_bindings: HashMap<ID, (Identifier, TypingContext)>,
    /// `Create` bindings defined in the current definition: keys are the bound variables, values
    /// are the correspinding clauses
    pub create_bindings: HashMap<ID, Vec<Clause>>,
    /// Tracks whether there have been changes during the current pass
    pub new_changes: bool,
    /// Tracks the maximal used identifier in the program
    pub max_id: usize,
}

impl RewriteState {
    pub fn get_let(&self, var: ID) -> Option<(Identifier, TypingContext)> {
        self.let_bindings.get(&var).cloned()
    }

    pub fn get_create_clause(&self, var: ID, xtor: &Identifier) -> Option<(Clause, usize)> {
        let clauses = self.create_bindings.get(&var)?;
        let position = clauses
            .iter()
            .position(|clause| clause.xtor == *xtor)
            .unwrap_or_else(|| {
                panic!(
                    "Could not find create clause for {}",
                    xtor.print_to_string(None)
                )
            });
        Some((clauses[position].clone(), position))
    }

    pub fn get_switch_info(
        &mut self,
        called_label: &Identifier,
        called_args: &TypingContext,
    ) -> Option<SwitchInfo> {
        let called_def_position = self
            .defs
            .iter()
            .position(|def| def.name == *called_label)
            .unwrap_or_else(|| {
                panic!(
                    "Could not find label {}",
                    called_label.print_to_string(None)
                )
            });
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
            .unwrap_or_else(|| {
                panic!(
                    "Could not find argument of Switch variable {}",
                    switch_var.print_to_string(None)
                )
            });
        let (tag, let_args) = self
            .let_bindings
            .get(&called_args.bindings[switch_var_position].var.id)?;

        // swap the body of the current Definition with a temporary placeholder
        let Statement::Switch(switch) = std::mem::take(&mut called_def.body) else {
            unreachable!("we already know that the body is a Switch")
        };
        let clause_position = switch
            .clauses
            .iter()
            .position(|clause| clause.xtor == *tag)
            .unwrap_or_else(|| {
                panic!(
                    "Could not find Switch Clause for {}",
                    tag.print_to_string(None)
                )
            });

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
        bound_var: &Identifier,
    ) -> (Identifier, Vec<usize>, Vec<ContextBinding>) {
        let name = fresh_identifier(
            &mut self.max_id,
            &("lift_".to_string()
                + &self.current_label.print_to_string(None)
                + "_"
                + &bound_var.print_to_string(None)
                + "_"
                + &clause.xtor.print_to_string(None)),
        );

        let mut free_vars = BTreeSet::new();
        clause.body.typed_free_vars(&mut free_vars);
        // we pick fresh names for the parameters of the lifted statement to keep all binders
        // unique
        let (arg_positions, (mut clause_subst, (mut clause_args, mut clause_fresh_bindings))): (
            Vec<_>,
            (Vec<_>, (Vec<_>, Vec<_>)),
        ) = clause
            .context
            .bindings
            .into_iter()
            .enumerate()
            .filter(|(_, binding)| free_vars.contains(binding))
            .map(|(position, binding)| {
                let fresh_binding = ContextBinding {
                    var: fresh_identifier(&mut self.max_id, &binding.var.name),
                    ..binding.clone()
                };
                (
                    position,
                    (
                        (binding.var.id, fresh_binding.var.clone()),
                        (binding, fresh_binding),
                    ),
                )
            })
            .unzip();
        for arg in &clause_args {
            free_vars.remove(arg);
        }
        let (mut subst, (free_vars, mut fresh_bindings)): (Vec<_>, (Vec<_>, Vec<_>)) = free_vars
            .into_iter()
            .map(|binding| {
                let fresh_binding = ContextBinding {
                    var: fresh_identifier(&mut self.max_id, &binding.var.name),
                    ..binding.clone()
                };
                (
                    (binding.var.id, fresh_binding.var.clone()),
                    (binding, fresh_binding),
                )
            })
            .unzip();
        let mut args = free_vars.clone();
        args.append(&mut clause_args);
        subst.append(&mut clause_subst);
        fresh_bindings.append(&mut clause_fresh_bindings);

        // we have to rewrite the Create whose Clause we lift to avoid duplication
        let create = self
            .create_bindings
            .get_mut(&bound_var.id)
            .unwrap_or_else(|| {
                panic!(
                    "Could not find create for variable {}",
                    bound_var.print_to_string(None)
                )
            });
        create[position].body = Rc::new(
            Call {
                label: name.clone(),
                args: args.into(),
            }
            .into(),
        );

        let def = Def {
            name: name.clone(),
            context: fresh_bindings.into(),
            body: Rc::unwrap_or_clone(clause.body).subst_sim(&subst),
        };
        self.defs.push(def);

        (name, arg_positions, free_vars)
    }

    pub fn lift_switch_clause(
        &mut self,
        switch_info: &mut SwitchInfo,
        label: &Identifier,
    ) -> (Identifier, Vec<usize>) {
        let clause = &mut switch_info.switch.clauses[switch_info.clause_position];
        let called_def = &self.defs[switch_info.called_def_position];

        let name = fresh_identifier(
            &mut self.max_id,
            &("lift_".to_string()
                + &label.print_to_string(None)
                + "_"
                + &switch_info.switch.var.print_to_string(None)
                + "_"
                + &clause.xtor.print_to_string(None)),
        );

        let mut free_vars = BTreeSet::new();
        clause.body.typed_free_vars(&mut free_vars);
        // we pick fresh names for the parameters of the lifted statement to keep all binders
        // unique
        let (arg_positions, (subst, (args, fresh_bindings))): (Vec<_>, (Vec<_>, (Vec<_>, Vec<_>))) =
            called_def
                .context
                .bindings
                .iter()
                .chain(clause.context.bindings.iter())
                .enumerate()
                .filter(|(_, binding)| free_vars.contains(binding))
                .map(|(position, binding)| {
                    let fresh_binding = ContextBinding {
                        var: fresh_identifier(&mut self.max_id, &binding.var.name),
                        ..binding.clone()
                    };
                    (
                        position,
                        (
                            (binding.var.id, fresh_binding.var.clone()),
                            (binding.clone(), fresh_binding),
                        ),
                    )
                })
                .unzip();

        // we have to rewrite the Switch whose Clause we lift to avoid duplication
        let body = std::mem::replace(
            &mut clause.body,
            Rc::new(
                Call {
                    label: name.clone(),
                    args: args.into(),
                }
                .into(),
            ),
        );

        let def = Def {
            name: name.clone(),
            context: fresh_bindings.into(),
            body: Rc::unwrap_or_clone(body).subst_sim(&subst),
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
