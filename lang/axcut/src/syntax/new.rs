use super::{
    context::context_vars,
    names::{filter_by_set, freshen},
    Clause, Statement, Ty, Var,
};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use printer::theme::ThemeExt;
use printer::tokens::{COLON, EQ, NEW, SEMI};
use printer::util::BracesExt;
use printer::{DocAllocator, Print};

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct New {
    pub var: Var,
    pub ty: Ty,
    pub context: Option<Vec<Var>>,
    pub clauses: Vec<Clause>,
    pub next: Rc<Statement>,
}

impl Print for New {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(NEW)
            .append(alloc.space())
            .append(&self.var)
            .append(alloc.space())
            .append(COLON)
            .append(alloc.space())
            .append(self.ty.print(cfg, alloc))
            .append(alloc.space())
            .append(EQ)
            .append(alloc.space())
            .append(self.context.print(cfg, alloc).parens())
            .append(self.clauses.print(cfg, alloc).braces_anno())
            .append(SEMI)
            .append(alloc.space())
            .append(self.next.print(cfg, alloc))
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
        mut context: Vec<Var>,
        used_vars: &mut HashSet<Var>,
    ) -> crate::syntax::Substitute {
        let mut free_vars_clauses = HashSet::new();
        self.clauses.free_vars(&mut free_vars_clauses);
        let mut free_vars_next = HashSet::new();
        self.next.free_vars(&mut free_vars_next);

        let context_next = filter_by_set(&context, &free_vars_next);
        let mut context_reordered = context.split_off(context_next.len());
        context_reordered.append(&mut context);
        let context_clauses = filter_by_set(&context_reordered, &free_vars_clauses);
        let mut context_next_freshened = freshen(
            &context_next,
            context_clauses.clone().into_iter().collect(),
            used_vars,
        );

        let mut full_context_freshened = context_next_freshened.clone();
        full_context_freshened.append(&mut context_clauses.clone());
        let mut full_context = context_next.clone();
        full_context.append(&mut context_clauses.clone());

        let rearrange = full_context_freshened
            .into_iter()
            .zip(full_context)
            .collect();

        let substitution_next: Vec<(Var, Var)> = context_next
            .into_iter()
            .zip(context_next_freshened.clone())
            .collect();
        let next_substituted = self.next.subst_sim(substitution_next.as_slice());

        context_next_freshened.push(self.var.clone());

        let clauses = self
            .clauses
            .into_iter()
            .map(
                |Clause {
                     xtor,
                     context,
                     case,
                 }| {
                    let mut extended_context = context_vars(&context);
                    extended_context.append(&mut context_clauses.clone());
                    crate::syntax::Clause {
                        xtor,
                        context,
                        case: case.linearize(extended_context, used_vars),
                    }
                },
            )
            .collect();

        crate::syntax::Substitute {
            rearrange,
            next: Rc::new(
                crate::syntax::New {
                    var: self.var,
                    ty: self.ty,
                    context: Some(context_clauses),
                    clauses,
                    next: next_substituted.linearize(context_next_freshened, used_vars),
                }
                .into(),
            ),
        }
    }
}
