use printer::{theme::ThemeExt, tokens::SWITCH, DocAllocator, Print};

use super::{print_clauses, Clause, Substitute};
use crate::syntax::{names::filter_by_set, Statement, Ty, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{fresh_var, Linearizing};
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Switch {
    pub var: Var,
    pub ty: Ty,
    pub clauses: Vec<Clause>,
    pub free_vars_clauses: Option<HashSet<Var>>,
}

impl Print for Switch {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(SWITCH)
            .append(alloc.space())
            .append(&self.var)
            .append(alloc.space())
            .append(print_clauses(&self.clauses, cfg, alloc))
    }
}

impl From<Switch> for Statement {
    fn from(value: Switch) -> Self {
        Statement::Switch(value)
    }
}

impl FreeVars for Switch {
    fn free_vars(mut self) -> (Self, HashSet<Var>) {
        let (clauses, vars_clauses) = self.clauses.free_vars();
        self.clauses = clauses;
        self.free_vars_clauses = Some(vars_clauses.clone());

        let mut vars = vars_clauses;
        vars.insert(self.var.clone());

        (self, vars)
    }
}

impl Subst for Switch {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Switch {
        self.var = self.var.subst_sim(subst);
        self.clauses = self.clauses.subst_sim(subst);
        self.free_vars_clauses = self.free_vars_clauses.subst_sim(subst);
        self
    }
}

impl Linearizing for Switch {
    type Target = Statement;
    fn linearize(mut self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let free_vars = std::mem::take(&mut self.free_vars_clauses)
            .expect("Free variables must be annotated before linearization");

        let new_context = filter_by_set(&context, &free_vars);
        let mut context_rearrange = new_context.clone();
        context_rearrange.push(self.var.clone());

        self.clauses = self
            .clauses
            .into_iter()
            .map(|mut clause| {
                let mut extended_context = new_context.clone();
                extended_context.append(&mut clause.context.vars());
                clause.body = clause.body.linearize(extended_context, used_vars);
                clause
            })
            .collect();

        if context == context_rearrange {
            self.into()
        } else {
            // if `self.var` is duplicated, pick a fresh one
            if new_context.contains(&self.var) {
                self.var = fresh_var(used_vars, &self.var);
            }

            let mut context_rearrange_freshened = new_context.clone();
            context_rearrange_freshened.push(self.var.clone());

            let rearrange = context_rearrange_freshened
                .into_iter()
                .zip(context_rearrange)
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(self.into()),
            }
            .into()
        }
    }
}
