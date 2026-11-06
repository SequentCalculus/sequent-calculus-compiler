//! This module defines explicit substitutions in AxCut.

use printer::theme::ThemeExt;
use printer::tokens::{COMMA, SEMI, SUBSTITUTE};
use printer::{DocAllocator, Print};

use crate::syntax::{ContextBinding, ID, Identifier, Prog, Statement};
use crate::traits::free_vars::FreeVars;
use crate::traits::steadfast::{
    CollectSteadfastConstraints, SteadfastConstraints, SubstituteSteadfastAnnotations,
};
use crate::traits::substitution::Subst;
use crate::traits::typed_free_vars::TypedFreeVars;

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

/// This module defines explicit substitutions in AxCut. They consist of a list of assignments of
/// new variables to old variables according to which the context is rearranged, and the remaining
/// statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Substitute {
    pub rearrange: Vec<(ContextBinding, Identifier)>,
    pub next: Rc<Statement>,
}

impl Print for Substitute {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let sep = alloc.text(COMMA).append(alloc.line());
        let rearrange = alloc.intersperse(
            self.rearrange.iter().map(|(new, old)| {
                new.var
                    .print(cfg, alloc)
                    .append(new.quantity.print(cfg, alloc))
                    .append(" := ")
                    .append(old.print(cfg, alloc))
                    .parens()
            }),
            sep,
        );
        alloc
            .keyword(SUBSTITUTE)
            .append(alloc.line())
            .append(rearrange)
            .nest(cfg.indent)
            .group()
            .append(SEMI)
            .append(alloc.hardline())
            .append(self.next.print(cfg, alloc).group())
    }
}

impl From<Substitute> for Statement {
    fn from(value: Substitute) -> Self {
        Statement::Substitute(value)
    }
}

impl FreeVars for Substitute {
    fn free_vars(mut self, vars: &mut HashSet<ID>) -> Self {
        self.next = self.next.free_vars(vars);

        for (new, old) in &self.rearrange {
            vars.insert(old.id);
            vars.remove(&new.var.id);
        }

        self
    }
}

impl TypedFreeVars for Substitute {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.next.typed_free_vars(vars);

        for (new, old) in &self.rearrange {
            vars.insert(ContextBinding {
                var: old.clone(),
                ..new.clone()
            });
            vars.remove(new);
        }
    }
}

impl Subst for Substitute {
    // this function is actually never called on the linearized version of AxCut containing
    // explicit substitutions; since the linearization pass does not generate fresh variables for
    // reorderings, we substitute into the left-hand sides as well, instead of considering these
    // bindings as proper shadowing, so that it stays obvious which rearrangments are reorderings
    fn subst_sim(mut self, subst: &[(ID, Identifier)]) -> Substitute {
        self.rearrange = self
            .rearrange
            .into_iter()
            .map(|(new, old)| (new.subst_sim(subst), old.subst_sim(subst)))
            .collect();
        self.next = self.next.subst_sim(subst);
        self
    }
}

impl CollectSteadfastConstraints for Substitute {
    fn collect_sf_constraints(
        &self,
        program: &Prog,
        context: &[ID],
        constraints: &mut SteadfastConstraints,
    ) {
        let next_context: Vec<ID> = self
            .rearrange
            .iter()
            .map(|(new, _old)| new.var.id)
            .collect();
        self.next
            .collect_sf_constraints(program, &next_context, constraints);

        for (new, old) in &self.rearrange {
            if new.var.id != old.id {
                constraints.vars_flow_together(new.var.id, old.id);
            }
        }

        for var in context {
            let occurences = self
                .rearrange
                .iter()
                .map(|(_new, old)| old.id)
                .filter(|id| *id == *var)
                .count();

            if occurences != 1 {
                constraints.quantity_constraints.insert(*var);
            }
        }
    }
}

impl SubstituteSteadfastAnnotations for Substitute {
    fn subst_sf_annotations(mut self, quantity_constraints: &HashSet<ID>) -> Self {
        let new_rearrange = Vec::with_capacity(self.rearrange.len());
        for (mut new, old) in std::mem::replace(&mut self.rearrange, new_rearrange) {
            new = new.subst_sf_annotations(quantity_constraints);
            self.rearrange.push((new, old));
        }
        self.next = self.next.subst_sf_annotations(quantity_constraints);
        self
    }
}
