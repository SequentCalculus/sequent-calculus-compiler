//! This module defines the binding of an xtor in AxCut.

use printer::theme::ThemeExt;
use printer::tokens::{COLON, EQ, LET, SEMI};
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{Chirality, ContextBinding, Name, Statement, Ty, TypingContext, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

/// This struct defines the binding of an xtor in AxCut. It consists of a variable to which to bind
/// the xtor, its type, the name of the xtor, its arguments, and the remaining statement. Moreover,
/// the free variables of the remaining statement can be annotated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Let {
    pub var: Var,
    pub ty: Ty,
    pub tag: Name,
    pub args: TypingContext,
    pub next: Rc<Statement>,
    pub free_vars_next: Option<HashSet<Var>>,
}

impl Print for Let {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let args = if self.args.bindings.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };

        alloc
            .keyword(LET)
            .append(alloc.space())
            .append(self.var.print(cfg, alloc))
            .append(COLON)
            .append(alloc.space())
            .append(self.ty.print(cfg, alloc))
            .append(alloc.space())
            .append(EQ)
            .append(alloc.space())
            .append(self.tag.print(cfg, alloc))
            .append(args.group())
            .append(SEMI)
            .append(alloc.hardline())
            .append(self.next.print(cfg, alloc).group())
    }
}

impl From<Let> for Statement {
    fn from(value: Let) -> Self {
        Statement::Let(value)
    }
}

impl FreeVars for Let {
    fn free_vars(mut self, vars: &mut HashSet<Var>) -> Self {
        self.next = self.next.free_vars(vars);
        self.free_vars_next = Some(vars.clone());

        vars.remove(&self.var);
        vars.extend(self.args.vars());

        self
    }
}

impl Subst for Let {
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Let {
        self.args = self.args.subst_sim(subst);
        self.next = self.next.subst_sim(subst);
        self.free_vars_next = self.free_vars_next.subst_sim(subst);
        self
    }
}

impl Linearizing for Let {
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
        // ... and the arguments of the xtor
        let mut context_rearrange = new_context.clone();
        context_rearrange
            .bindings
            .extend(self.args.bindings.clone());

        let new_binding = ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Prd,
            ty: self.ty.clone(),
        };

        if context == context_rearrange {
            // if the context is exactly right already, we simply linearize the remaining statement
            // with the additional binding for the xtor
            new_context.bindings.push(new_binding);
            self.next = self.next.linearize(new_context, used_vars);
            self.into()
        } else {
            // otherwise we pick fresh names for duplicated variables in the arguments ...
            self.args = self.args.freshen(new_context.vars_set(), used_vars);

            // ...  via the rearrangement in an explicit substitution
            let mut context_rearrange_freshened = new_context.clone();
            context_rearrange_freshened
                .bindings
                .extend(self.args.bindings.clone());

            // linearize the remaining statement with the additional binding for the xtor
            new_context.bindings.push(new_binding);
            self.next = self.next.linearize(new_context, used_vars);

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
