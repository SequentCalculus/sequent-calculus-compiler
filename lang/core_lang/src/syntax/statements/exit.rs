//! This module defines the exit statement in Core.

use printer::tokens::EXIT;
use printer::*;

use crate::syntax::*;
use crate::traits::*;

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

/// This struct defines the exit statement in Core. It consists of a term for the exit code and the
/// type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exit {
    /// The exit code
    pub arg: Rc<Term<Prd>>,
    /// The type
    pub ty: Ty,
}

impl Typed for Exit {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl Print for Exit {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        alloc
            .keyword(EXIT)
            .append(alloc.space())
            .append(self.arg.print(cfg, alloc))
    }
}

impl From<Exit> for Statement {
    fn from(value: Exit) -> Self {
        Statement::Exit(value)
    }
}

impl Subst for Exit {
    type Target = Exit;
    fn subst_sim(
        mut self,
        prod_subst: &[(Ident, Term<Prd>)],
        cons_subst: &[(Ident, Term<Cns>)],
    ) -> Self::Target {
        self.arg = self.arg.subst_sim(prod_subst, cons_subst);

        self
    }
}

impl TypedFreeVars for Exit {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.arg.typed_free_vars(vars);
    }
}

impl Uniquify for Exit {
    fn uniquify(mut self, state: &mut UniquifyState) -> Exit {
        self.arg = self.arg.uniquify(state);

        self
    }
}

impl Focusing for Exit {
    type Target = FsStatement;
    // focus(exit p) = bind(p)[λa.exit a]
    fn focus(self, used_vars: &mut HashSet<Ident>) -> FsStatement {
        let cont = Box::new(Box::new(
            |binding: ContextBinding, _: &mut HashSet<Ident>| FsExit { var: binding.var }.into(),
        ));
        Rc::unwrap_or_clone(self.arg).bind(cont, used_vars)
    }
}

/// This struct defines the focused version of the [`Exit`] statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsExit {
    /// The exit code (always a variable here)
    pub var: Ident,
}

impl FsExit {
    /// This fcuntion constructs an exit statement from a given variable.
    #[allow(clippy::self_named_constructors)]
    pub fn exit(var: Ident) -> Self {
        FsExit { var }
    }
}

impl Print for FsExit {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        alloc
            .keyword(EXIT)
            .append(alloc.space())
            .append(self.var.print(cfg, alloc))
    }
}

impl From<FsExit> for FsStatement {
    fn from(value: FsExit) -> Self {
        FsStatement::Exit(value)
    }
}

impl SubstVar for FsExit {
    type Target = FsExit;
    fn subst_sim(mut self, subst: &[(Ident, Ident)]) -> Self::Target {
        self.var = self.var.subst_sim(subst);

        self
    }
}

impl TypedFreeVars for FsExit {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        vars.insert(ContextBinding {
            var: self.var.clone(),
            chi: Chirality::Prd,
            ty: Ty::I64,
        });
    }
}
