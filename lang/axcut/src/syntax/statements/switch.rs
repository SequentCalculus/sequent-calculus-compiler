use printer::{theme::ThemeExt, tokens::SWITCH, DocAllocator, Print};

use super::Substitute;
use crate::syntax::clause::print_clauses;
use crate::syntax::{names::filter_by_set, Clause, Statement, Ty, Var};
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

impl Linearizing for Switch {
    type Target = Statement;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars = HashSet::new();
        self.clauses.free_vars(&mut free_vars);

        let new_context = filter_by_set(&context, &free_vars);
        let mut context_rearrange = new_context.clone();
        context_rearrange.push(self.var.clone());

        // If the condition is true, then `context != context_rearrange`, since then `self.var`
        // is duplicated. Hence, if `context == context_rearrange`, then `var == self.var`.
        let var = if new_context.contains(&self.var) {
            fresh_var(used_vars, &self.var)
        } else {
            self.var.clone()
        };

        let clauses = self
            .clauses
            .into_iter()
            .map(
                |Clause {
                     xtor,
                     context,
                     case,
                 }| {
                    let mut extended_context = new_context.clone();
                    extended_context.append(&mut context.vars());
                    Clause {
                        xtor,
                        context,
                        case: case.linearize(extended_context, used_vars),
                    }
                },
            )
            .collect();
        let switch = Switch {
            var: var.clone(),
            ty: self.ty,
            clauses,
        }
        .into();

        if context == context_rearrange {
            switch
        } else {
            let mut context_rearrange_freshened = new_context.clone();
            context_rearrange_freshened.push(var);

            let rearrange = context_rearrange_freshened
                .into_iter()
                .zip(context_rearrange)
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(switch),
            }
            .into()
        }
    }
}
