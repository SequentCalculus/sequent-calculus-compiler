use super::{Clause, Statement};
use crate::syntax::context::{context_vars, filter_by_set, freshen};
use crate::syntax::{stringify_and_join, ContextBinding, Polarity, Ty, TypingContext, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct New {
    pub var: Var,
    pub ty: Ty,
    pub clauses: Vec<Clause>,
    pub next: Rc<Statement>,
}

impl std::fmt::Display for New {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clauses = stringify_and_join(&self.clauses, "\n    ");
        write!(
            f,
            "new {} : {} = {{\n    {} }};\n  {}",
            self.var, self.ty, clauses, self.next
        )
    }
}

impl From<New> for Statement {
    fn from(value: New) -> Self {
        Statement::New(value)
    }
}

impl FreeVars for New {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.next.free_vars(vars);
        vars.remove(&self.var);
        self.clauses.free_vars(vars);
    }
}

impl Subst for New {
    type Target = New;

    fn subst_sim(self, subst: &[(Var, Var)]) -> New {
        New {
            clauses: self.clauses.subst_sim(subst),
            next: self.next.subst_sim(subst),
            ..self
        }
    }
}

impl UsedBinders for New {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        used.insert(self.var.clone());
        self.clauses.used_binders(used);
        self.next.used_binders(used);
    }
}

impl Linearizing for New {
    type Target = crate::syntax::Substitute;
    fn linearize(
        self,
        context: TypingContext,
        used_vars: &mut HashSet<Var>,
    ) -> crate::syntax::Substitute {
        let mut free_vars_clauses = HashSet::new();
        self.clauses.free_vars(&mut free_vars_clauses);
        let mut free_vars_next = HashSet::new();
        self.next.free_vars(&mut free_vars_next);

        let context_clauses = filter_by_set(&context, &free_vars_clauses);
        let context_next = filter_by_set(&context, &free_vars_next);
        let mut context_next_freshened = freshen(context_next.clone(), used_vars);

        let mut full_context_freshened = context_next_freshened.clone();
        full_context_freshened.append(&mut context_clauses.clone());
        let mut full_context = context_next.clone();
        full_context.append(&mut context_clauses.clone());

        let rearrange = full_context_freshened
            .into_iter()
            .zip(full_context)
            .collect();

        let substitution_next: Vec<(Var, Var)> = context_vars(&context_next)
            .into_iter()
            .zip(context_vars(&context_next_freshened))
            .collect();
        let next_substituted = self.next.subst_sim(substitution_next.as_slice());

        context_next_freshened.push(ContextBinding {
            var: self.var.clone(),
            pol: Polarity::Cns,
            ty: self.ty.clone(),
        });

        let clauses = self
            .clauses
            .into_iter()
            .map(|Clause { env, case }| {
                let mut extended_context = env.clone();
                extended_context.append(&mut context_clauses.clone());
                crate::syntax::Clause {
                    env,
                    case: case.linearize(extended_context, used_vars),
                }
            })
            .collect();

        crate::syntax::Substitute {
            rearrange,
            next: Rc::new(
                crate::syntax::New {
                    var: self.var,
                    ty: self.ty,
                    env: context_clauses,
                    clauses,
                    next: next_substituted.linearize(context_next_freshened, used_vars),
                }
                .into(),
            ),
        }
    }
}
