use super::{Clause, Statement};
use crate::syntax::{context::context_vars, names::filter_by_set, stringify_and_join, Ty, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{fresh_var, Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Switch {
    pub var: Var,
    pub ty: Ty,
    pub clauses: Vec<Clause>,
}

impl std::fmt::Display for Switch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clauses = stringify_and_join(&self.clauses, "\n    ");
        write!(f, "switch {} {{\n    {} }}", self.var, clauses)
    }
}

impl From<Switch> for Statement {
    fn from(value: Switch) -> Self {
        Statement::Switch(value)
    }
}

impl FreeVars for Switch {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.clauses.free_vars(vars);
        vars.insert(self.var.clone());
    }
}

impl Subst for Switch {
    type Target = Switch;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Switch {
        Switch {
            var: self.var.subst_sim(subst),
            clauses: self.clauses.subst_sim(subst),
            ..self
        }
    }
}

impl UsedBinders for Switch {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.clauses.used_binders(used);
    }
}

impl Linearizing for Switch {
    type Target = crate::syntax::Substitute;
    fn linearize(
        self,
        context: Vec<Var>,
        used_vars: &mut HashSet<Var>,
    ) -> crate::syntax::Substitute {
        let mut free_vars = HashSet::new();
        self.clauses.free_vars(&mut free_vars);

        let new_context = filter_by_set(&context, &free_vars);

        let mut full_context_freshened = new_context.clone();
        let fresh_var = if new_context.contains(&self.var) {
            fresh_var(used_vars, &self.var)
        } else {
            self.var.clone()
        };
        full_context_freshened.push(fresh_var.clone());

        let mut full_context = new_context.clone();
        full_context.push(self.var);

        let rearrange = full_context_freshened
            .into_iter()
            .zip(full_context)
            .collect();

        let clauses = self
            .clauses
            .into_iter()
            .map(|Clause { context, case }| {
                let mut extended_context = new_context.clone();
                extended_context.append(&mut context_vars(&context));
                crate::syntax::Clause {
                    context,
                    case: case.linearize(extended_context, used_vars),
                }
            })
            .collect();

        crate::syntax::Substitute {
            rearrange,
            next: Rc::new(
                crate::syntax::Switch {
                    var: fresh_var,
                    clauses,
                }
                .into(),
            ),
        }
    }
}
