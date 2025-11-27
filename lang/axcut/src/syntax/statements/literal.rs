//! This module defines integer literals in AxCut.

use printer::theme::ThemeExt;
use printer::tokens::{LEFT_ARROW, LIT, SEMI};
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{Chirality, ContextBinding, Statement, Ty, TypingContext, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;
use crate::traits::typed_free_vars::TypedFreeVars;

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

/// This struct defines integer literals in AxCut. They consist of the literal, the variable the
/// literal is bound to, and the remaining statement. Moreover, the free variables of the remaining
/// statement can be annotated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub lit: i64,
    pub var: Var,
    pub next: Rc<Statement>,
    pub free_vars_next: Option<HashSet<Var>>,
}

impl Print for Literal {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(LIT)
            .append(alloc.space())
            .append(self.var.print(cfg, alloc))
            .append(alloc.space())
            .append(LEFT_ARROW)
            .append(alloc.space())
            .append(format!("{}", self.lit))
            .append(SEMI)
            .append(alloc.hardline())
            .append(self.next.print(cfg, alloc).group())
    }
}

impl From<Literal> for Statement {
    fn from(value: Literal) -> Self {
        Statement::Literal(value)
    }
}

impl FreeVars for Literal {
    fn free_vars(mut self, vars: &mut HashSet<Var>) -> Self {
        self.next = self.next.free_vars(vars);
        self.free_vars_next = Some(vars.clone());

        vars.remove(&self.var);

        self
    }
}

impl Subst for Literal {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Literal {
        self.next = self.next.subst_sim(subst);
        self.free_vars_next = self.free_vars_next.subst_sim(subst);
        self
    }
}

impl TypedFreeVars for Literal {
    fn typed_free_vars(&self) -> BTreeSet<ContextBinding> {
        self.next
            .typed_free_vars()
            .into_iter()
            .filter(|bnd| bnd.var != self.var)
            .collect()
    }
}

impl Linearizing for Literal {
    type Target = Statement;
    /// # Panics
    ///
    /// In this implementation of [`Linearizing::linearize`] a panic is caused if the free
    /// variables of the remaining statement are not annotated.
    fn linearize(mut self, context: TypingContext, used_vars: &mut HashSet<Var>) -> Statement {
        let free_vars = std::mem::take(&mut self.free_vars_next)
            .expect("Free variables must be annotated before linearization");

        // the new context consists of the context for the remaining statement ...
        let mut new_context = context.filter_by_set(&free_vars);
        let context_rearrange = new_context.clone();
        // ... and the variable the literal is bound to
        let new_binding = ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Ext,
            ty: Ty::I64,
        };
        new_context.bindings.push(new_binding);

        // linearize the remaining statement
        self.next = self.next.linearize(new_context, used_vars);

        if context == context_rearrange {
            // if the context is exactly right already, we do not have to do anything
            self.into()
        } else {
            // otherwise we insert an explicit substitution
            let rearrange = context_rearrange
                .bindings
                .clone()
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
