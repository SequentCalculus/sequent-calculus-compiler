//! This module defines the creation of a closure in AxCut.

use printer::theme::ThemeExt;
use printer::tokens::{COLON, COMMA, CREATE, EQ, SEMI};
use printer::{DocAllocator, Print};

use super::{Clause, Substitute, print_clauses};
use crate::syntax::{Chirality, ContextBinding, Statement, Ty, TypingContext, Var};

use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;
use crate::traits::typed_free_vars::TypedFreeVars;

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

/// This struct defines the creation of a closure in AxCut. It consists of a variable to which to
/// bind the closure, its type, a list of clauses (one for each xtor in the type declaration), and
/// the remaining statement. Moreover, the closure environment can be annotated as is done by the
/// linearization procedure. Additionally, the free variables of the clauses and of the remaining
/// statement can be annotated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Create {
    pub var: Var,
    pub ty: Ty,
    /// Closure environment
    pub context: Option<TypingContext>,
    pub clauses: Vec<Clause>,
    pub free_vars_clauses: Option<HashSet<Var>>,
    pub next: Rc<Statement>,
    pub free_vars_next: Option<HashSet<Var>>,
}

impl Print for Create {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let context = if let Some(ref context) = self.context {
            let sep = alloc.text(COMMA).append(alloc.line());
            alloc
                .intersperse(
                    context
                        .bindings
                        .iter()
                        .map(|binding| binding.var.print(cfg, alloc).group()),
                    sep,
                )
                .parens()
        } else {
            alloc.nil()
        };

        alloc
            .keyword(CREATE)
            .append(alloc.space())
            .append(self.var.print(cfg, alloc))
            .append(COLON)
            .append(alloc.space())
            .append(self.ty.print(cfg, alloc))
            .append(alloc.space())
            .append(EQ)
            .append(alloc.space())
            .append(context.group())
            .append(print_clauses(&self.clauses, cfg, alloc))
            .append(SEMI)
            .append(alloc.hardline())
            .append(self.next.print(cfg, alloc).group())
    }
}

impl From<Create> for Statement {
    fn from(value: Create) -> Self {
        Statement::Create(value)
    }
}

impl FreeVars for Create {
    fn free_vars(mut self, vars: &mut HashSet<Var>) -> Self {
        self.next = self.next.free_vars(vars);
        self.free_vars_next = Some(vars.clone());

        let mut vars_clauses = HashSet::new();
        self.clauses = self.clauses.free_vars(&mut vars_clauses);
        self.free_vars_clauses = Some(vars_clauses.clone());

        vars.remove(&self.var);
        vars.extend(vars_clauses);

        self
    }
}

impl Subst for Create {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Create {
        self.context = self.context.subst_sim(subst);
        self.clauses = self.clauses.subst_sim(subst);
        self.next = self.next.subst_sim(subst);
        self.free_vars_clauses = self.free_vars_clauses.subst_sim(subst);
        self.free_vars_next = self.free_vars_next.subst_sim(subst);
        self
    }
}

impl TypedFreeVars for Create {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding> {
        let mut bindings = self.next.typed_free_vars();
        bindings.retain(|bnd| bnd.var != self.var);
        for clause in self.clauses.iter() {
            bindings.extend(clause.typed_free_vars());
        }
        bindings
    }
}

impl Linearizing for Create {
    type Target = Statement;
    /// # Panics
    ///
    /// In this implementation of [`Linearizing::linearize`] a panic is caused if the free
    /// variables of the clauses and the remaining statement are not annotated.
    fn linearize(mut self, mut context: TypingContext, used_vars: &mut HashSet<Var>) -> Statement {
        let free_vars_clauses = std::mem::take(&mut self.free_vars_clauses)
            .expect("Free variables must be annotated before linearization");
        let free_vars_next = std::mem::take(&mut self.free_vars_next)
            .expect("Free variables must be annotated before linearization");

        // back up the current context
        let context_clone = context.clone();
        // calculate the context for the remaining statement
        let mut context_next = context.filter_by_set(&free_vars_next);
        // the context for the remaining statement will come before the closure environment in the
        // explicit substitution; as `filter_by_set` tries to retain the positions in the order it
        // gets passed the context, we split off the number of variables for the remaining context
        // and put them to the end when calculating the closure environment; then `filter_by_set`
        // sees the position where the closure environment will start later at the very beginning
        // (I know, it's a bit consfusing)
        let mut context_reordered = TypingContext {
            bindings: context.bindings.split_off(context_next.bindings.len()),
        };
        context_reordered.bindings.append(&mut context.bindings);
        // calculate the closure environment needed by the clauses
        let context_clauses = context_reordered.filter_by_set(&free_vars_clauses);

        // each clause is linearized with the closure environment appended to the bindings
        self.clauses = self
            .clauses
            .into_iter()
            .map(|mut clause| {
                let mut extended_context = clause.context.clone();
                extended_context
                    .bindings
                    .extend(context_clauses.bindings.clone());
                clause.body = clause.body.linearize(extended_context, used_vars);
                clause
            })
            .collect();

        // the new context consists of the context for the remaining statement ...
        let mut context_rearrange = context_next.clone();
        // ... and the closure environment
        context_rearrange
            .bindings
            .extend(context_clauses.bindings.clone());

        let new_binding = ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Cns,
            ty: self.ty.clone(),
        };

        if context_clone == context_rearrange {
            // if the context is exactly right already, we simply annotate the closure environment
            // ...
            self.context = Some(context_clauses);

            // ... and linearize the remaining statement with the additional binding for the
            // closure
            context_next.bindings.push(new_binding);
            self.next = self.next.linearize(context_next, used_vars);

            self.into()
        } else {
            // otherwise we pick fresh names for duplicated variables in the remaining statement ...
            let mut context_next_freshened =
                context_next.freshen(context_clauses.vars_set(), used_vars);

            // ...  via the rearrangement in an explicit substitution
            let mut context_rearrange_freshened = context_next_freshened.clone();
            context_rearrange_freshened
                .bindings
                .extend(context_clauses.bindings.clone());
            let rearrange = context_rearrange_freshened
                .bindings
                .into_iter()
                .zip(context_rearrange.into_iter_vars())
                .collect();

            // annotate the closure environment
            self.context = Some(context_clauses);

            // since we have picked fresh names in the remaining statement, we have to rename in it
            // accordingly
            let substitution_next: Vec<(Var, Var)> = context_next
                .into_iter_vars()
                .zip(context_next_freshened.vars())
                .collect();
            self.next = self.next.subst_sim(substitution_next.as_slice());

            // linearize the remaining statement with the additional binding for the closure
            context_next_freshened.bindings.push(new_binding);
            self.next = self.next.linearize(context_next_freshened, used_vars);

            Substitute {
                rearrange,
                next: Rc::new(self.into()),
            }
            .into()
        }
    }
}
