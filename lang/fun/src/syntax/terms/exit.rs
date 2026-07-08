//! This module defines the exit statement in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::EXIT;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::ConstraintBank;
use crate::typing::inference::Inference;
use crate::typing::*;

use std::collections::HashMap;
use std::{collections::HashSet, rc::Rc};

/// This struct defines the exit statement in Fun. It consists of a term for the exit code, and
/// after typechecking also of the inferred type, which can be arbitrary.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Exit {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The exit code
    pub arg: Rc<Term>,
    /// The (inferred) type of the term
    pub ty: Option<Ty>,
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

impl Inference for Exit {
    fn gather_constraints(
        &mut self,
        constraint_bank: &mut ConstraintBank,
        context: &TypingContext,
        ty_var: Ty,
    ) -> Result<(), Error> {
        self.ty = Some(ty_var);

        self.arg
            .gather_constraints(constraint_bank, context, Ty::mk_i64())
    }

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<u32, usize>,
    ) -> Result<(), Error> {
        self.arg
            .insert_inferred_type(mappings, symbol_table, choices)?;

        match &mut self.ty {
            Some(ty_var) => {
                ty_var.mut_subst_ty(mappings);
                ty_var.check(&Some(self.span), symbol_table)
            }
            None => panic!(
                "The Type of the term {:?} is not set after type inference",
                self
            ),
        }
    }
}

impl UsedBinders for Exit {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.arg.used_binders(used);
    }
}
