//! This module defines the exit statement in Fun.

use codespan::Span;
use derivative::Derivative;
use printer::tokens::EXIT;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::*;

use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

/// This struct defines the exit statement in Fun. It consists of a term for the exit code, and
/// after typechecking also of the inferred type, which can be arbitrary.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Exit {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The exit code
    pub arg: Rc<Term>,
    /// The (inferred) type of the term
    pub ty: Option<Ty>,
}

impl SubstType for Exit {
    fn subst_ty(mut self, mappings: &HashMap<Name, Ty>) -> Self {
        self.arg = self.arg.subst_ty(mappings);
        self.ty = self.ty.subst_ty(mappings);
        self
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

impl OptTyped for Exit {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl From<Exit> for Term {
    fn from(value: Exit) -> Self {
        Term::Exit(value)
    }
}

impl Check for Exit {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        self.arg = self.arg.check(symbol_table, context, &Ty::mk_i64())?;

        self.ty = Some(expected.clone());
        Ok(self)
    }
}

impl UsedBinders for Exit {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.arg.used_binders(used);
    }
}
