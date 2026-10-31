//! This module defines the call of a top-level function in AxCut.

use printer::Print;

use super::Substitute;
use crate::syntax::{ContextBinding, ID, Identifier, Prog, Statement, TypingContext};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::steadfast::{
    CollectSteadfastConstraints, SteadfastConstraints, SubstituteSteadfastAnnotations,
};
use crate::traits::substitution::Subst;
use crate::traits::typed_free_vars::TypedFreeVars;

use std::{
    collections::{BTreeSet, HashSet},
    rc::Rc,
};

/// This struct defines the call of a top-level function in AxCut. It consists of the name of the
/// top-level function to call and the arguments. After linearization, the arguments are
/// immaterial, because the context then has to exactly fit the signature of the top-level
/// function anyway.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub label: Identifier,
    pub args: TypingContext,
}

impl Print for Call {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.label
            .print(cfg, alloc)
            .append(self.args.print(cfg, alloc).parens().group())
    }
}

impl From<Call> for Statement {
    fn from(value: Call) -> Self {
        Statement::Call(value)
    }
}

impl FreeVars for Call {
    fn free_vars(self, vars: &mut HashSet<ID>) -> Self {
        vars.extend(self.args.bindings.iter().map(|binding| binding.var.id));
        self
    }
}

impl TypedFreeVars for Call {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        vars.extend(self.args.bindings.iter().cloned());
    }
}

impl Subst for Call {
    fn subst_sim(mut self, subst: &[(ID, Identifier)]) -> Call {
        self.args = self.args.subst_sim(subst);
        self
    }
}

impl Linearizing for Call {
    type Target = Statement;
    fn linearize(mut self, context: TypingContext, max_id: &mut ID) -> Statement {
        let args = std::mem::take(&mut self.args.bindings).into();

        // the context must consist of the arguments for the top-level function
        if context == args {
            // if the context is exactly right already, we do not have to do anything
            self.into()
        } else {
            // otherwise we pick fresh names for duplicated variables via an explicit substitution
            let freshened_context = args.freshen(HashSet::new(), max_id);
            let rearrange = freshened_context
                .bindings
                .into_iter()
                .zip(args.into_iter_vars())
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(self.into()),
            }
            .into()
        }
    }
}

impl CollectSteadfastConstraints for Call {
    fn collect_sf_constraints(
        &self,
        program: &Prog,
        context: &[ID],
        constraints: &mut SteadfastConstraints,
    ) {
        let params = program
            .find_def(&self.label)
            .expect("label refers to existing def");

        constraints.contexts_flow_together(context, &params.context.ids());
    }
}

impl SubstituteSteadfastAnnotations for Call {
    fn subst_sf_annotations(mut self, quantity_constraints: &HashSet<ID>) -> Self {
        self.args = self.args.subst_sf_annotations(quantity_constraints);
        self
    }
}
