//! This module defines the pattern matching on an xtor in AxCut.

use printer::{DocAllocator, Print, theme::ThemeExt, tokens::SWITCH};

use super::{Clause, Substitute, print_clauses};
use crate::syntax::{
    Chirality, ContextBinding, Statement, Ty, TypingContext, Var, names::filter_by_set,
};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{Linearizing, fresh_var};
use crate::traits::substitution::Subst;
use crate::traits::typed_free_vars::TypedFreeVars;

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

/// This struct defines the pattern matching on an xtor in AxCut. It consists of the variable to
/// match on, its type, and a list of clauses (one for each xtor in the type declaration).
/// Moreover, the free variables of the clauses can be annotated.
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
            .append(self.var.print(cfg, alloc))
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
    fn free_vars(mut self, vars: &mut HashSet<Var>) -> Self {
        self.clauses = self.clauses.free_vars(vars);
        self.free_vars_clauses = Some(vars.clone());

        vars.insert(self.var.clone());

        self
    }
}

impl TypedFreeVars for Switch {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.clauses.typed_free_vars(vars);
        vars.insert(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Prd,
            ty: self.ty.clone(),
        });
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
    /// # Panics
    ///
    /// In this implementation of [`Linearizing::linearize`] a panic is caused if the free
    /// variables of the clauses are not annotated.
    fn linearize(mut self, context: TypingContext, used_vars: &mut HashSet<Var>) -> Statement {
        let free_vars = std::mem::take(&mut self.free_vars_clauses)
            .expect("Free variables must be annotated before linearization");

        // the new context consists of the contetx for the clauses ...
        let new_context = context.filter_by_set(&free_vars);
        // ... followed by the variable of the xtor matched on
        let mut context_rearrange = new_context.clone();
        let xtor_binding = ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Prd,
            ty: self.ty.clone(),
        };
        context_rearrange.bindings.push(xtor_binding);

        // each clause is linearized with the context for the clauses prepended to the bindings
        self.clauses = self
            .clauses
            .into_iter()
            .map(|mut clause| {
                let mut extended_context = new_context.clone();
                extended_context
                    .bindings
                    .extend(clause.context.bindings.clone());
                clause.body = clause.body.linearize(extended_context, used_vars);
                clause
            })
            .collect();

        if context == context_rearrange {
            // if the context is exactly right already, we do not have to do anything
            self.into()
        } else {
<<<<<<< HEAD
            // otherwise we pick a fresh name for the variable matched on if it is duplicated ...
            if new_context.vars_set().contains(&self.var) {
=======
            // , pick a fresh one
            // otherwise we pick a fresh name for the matched on variable if it is duplicated ...
            if new_context.vars().contains(&self.var) {
                let old = self.var.clone();
>>>>>>> bb1112c1 (fixed but in linearize switch)
                self.var = fresh_var(used_vars, &self.var);
            }

            // ... via an explicit substitution
<<<<<<< HEAD
            let mut context_rearrange_freshened = new_context;
            let new_xtor_binding = ContextBinding {
                var: self.var.clone(),
                chi: Chirality::Prd,
                ty: self.ty.clone(),
            };
            context_rearrange_freshened.bindings.push(new_xtor_binding);
=======
            let mut context_rearrange_freshened = new_context.clone();
            let new_binding = ContextBinding {
                var: self.var.clone(),
                ty: self.ty.clone(),
                chi: Chirality::Cns,
            };

            context_rearrange_freshened.bindings.push(new_binding);
>>>>>>> bb1112c1 (fixed but in linearize switch)

            let rearrange = context_rearrange_freshened
                .bindings
                .into_iter()
                .zip(context_rearrange.into_iter_vars())
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(self.into()),
            }
            .into()
        }
    }
}
