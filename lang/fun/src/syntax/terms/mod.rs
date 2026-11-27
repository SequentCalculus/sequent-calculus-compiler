//! This module defines the terms of Fun.

pub mod call;
pub mod case;
pub mod clause;
pub mod constructor;
pub mod destructor;
pub mod exit;
pub mod goto;
pub mod ifc;
pub mod label;
pub mod r#let;
pub mod literal;
pub mod new;
pub mod op;
pub mod paren;
pub mod print;
pub mod var;

pub use call::*;
pub use case::*;
pub use clause::*;
pub use constructor::*;
pub use destructor::*;
pub use exit::*;
pub use goto::*;
pub use ifc::*;
pub use label::*;
pub use r#let::*;
pub use literal::*;
pub use new::*;
pub use op::*;
pub use paren::*;
pub use print::*;
pub use var::*;

use printer::Print;

use crate::{
    syntax::names::Var,
    traits::{OptTyped, used_binders::UsedBinders},
    typing::{
        errors::Error,
        inference::{TypeInference, TypeVarMapping, World},
        symbol_table::SymbolTable,
    },
};

use super::{context::TypingContext, types::Ty};

use std::collections::HashSet;

/// This enum defines the terms of Fun. It contains one variant for each construct which simply
/// wraps the struct defining the corresponding construct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    /// Variable or Covariable
    XVar(XVar),
    /// Integer literal
    Lit(Lit),
    /// Arithmetic binary operations
    Op(Op),
    /// Conditional comparing two integers
    IfC(IfC),
    /// Printing an integer
    PrintI64(PrintI64),
    /// Let-binding of a term
    Let(Let),
    /// Call of a top-level function
    Call(Call),
    /// Constructor of a data type
    Constructor(Constructor),
    /// Destructor of a codata type
    Destructor(Destructor),
    /// Pattern match for a data type
    Case(Case),
    /// Copattern match for a codata type
    New(New),
    /// Control operator for capturing current continuation/program context
    Label(Label),
    /// control operator for invoking a captured continuation/program context
    Goto(Goto),
    /// Exiting the program
    Exit(Exit),
    /// Parethesized term
    Paren(Paren),
}

impl OptTyped for Term {
    fn get_type(&self) -> Option<Ty> {
        match self {
            Term::XVar(var) => var.get_type(),
            Term::Lit(lit) => lit.get_type(),
            Term::Op(op) => op.get_type(),
            Term::IfC(ifc) => ifc.get_type(),
            Term::PrintI64(print) => print.get_type(),
            Term::Let(lt) => lt.get_type(),
            Term::Call(call) => call.get_type(),
            Term::Constructor(ctor) => ctor.get_type(),
            Term::Destructor(dtor) => dtor.get_type(),
            Term::Case(case) => case.get_type(),
            Term::New(new) => new.get_type(),
            Term::Goto(goto) => goto.get_type(),
            Term::Label(label) => label.get_type(),
            Term::Exit(exit) => exit.get_type(),
            Term::Paren(paren) => paren.get_type(),
        }
    }
}

impl Print for Term {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Term::XVar(var) => var.print(cfg, alloc),
            Term::Lit(lit) => lit.print(cfg, alloc),
            Term::Op(op) => op.print(cfg, alloc),
            Term::IfC(ifc) => ifc.print(cfg, alloc),
            Term::PrintI64(print) => print.print(cfg, alloc),
            Term::Let(r#let) => r#let.print(cfg, alloc),
            Term::Call(call) => call.print(cfg, alloc),
            Term::Constructor(constructor) => constructor.print(cfg, alloc),
            Term::Destructor(destructor) => destructor.print(cfg, alloc),
            Term::Case(case) => case.print(cfg, alloc),
            Term::New(new) => new.print(cfg, alloc),
            Term::Goto(goto) => goto.print(cfg, alloc),
            Term::Label(label) => label.print(cfg, alloc),
            Term::Exit(exit) => exit.print(cfg, alloc),
            Term::Paren(paren) => paren.print(cfg, alloc),
        }
    }
}

impl TypeInference for Term {
    fn collect_constraints(
        &mut self,
        state: &mut crate::typing::inference::TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        match self {
            Term::XVar(xvar) => xvar.collect_constraints(state, context, expected_ty),
            Term::Lit(lit) => lit.collect_constraints(state, context, expected_ty),
            Term::Op(op) => op.collect_constraints(state, context, expected_ty),
            Term::IfC(if_c) => if_c.collect_constraints(state, context, expected_ty),
            Term::PrintI64(print_i64) => print_i64.collect_constraints(state, context, expected_ty),
            Term::Let(let_block) => let_block.collect_constraints(state, context, expected_ty),
            Term::Call(call) => call.collect_constraints(state, context, expected_ty),
            Term::Constructor(constructor) => {
                constructor.collect_constraints(state, context, expected_ty)
            }
            Term::Destructor(destructor) => {
                destructor.collect_constraints(state, context, expected_ty)
            }
            Term::Case(case) => case.collect_constraints(state, context, expected_ty),
            Term::New(new_block) => new_block.collect_constraints(state, context, expected_ty),
            Term::Label(label) => label.collect_constraints(state, context, expected_ty),
            Term::Goto(goto) => goto.collect_constraints(state, context, expected_ty),
            Term::Exit(exit) => exit.collect_constraints(state, context, expected_ty),
            Term::Paren(paren) => paren.collect_constraints(state, context, expected_ty),
        }
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        match self {
            Term::XVar(xvar) => xvar.insert_inferred_types(type_vars, symbol_table, world),
            Term::Lit(lit) => lit.insert_inferred_types(type_vars, symbol_table, world),
            Term::Op(op) => op.insert_inferred_types(type_vars, symbol_table, world),
            Term::IfC(if_c) => if_c.insert_inferred_types(type_vars, symbol_table, world),
            Term::PrintI64(print_i64) => {
                print_i64.insert_inferred_types(type_vars, symbol_table, world)
            }
            Term::Let(let_block) => let_block.insert_inferred_types(type_vars, symbol_table, world),
            Term::Call(call) => call.insert_inferred_types(type_vars, symbol_table, world),
            Term::Constructor(constructor) => {
                constructor.insert_inferred_types(type_vars, symbol_table, world)
            }
            Term::Destructor(destructor) => {
                destructor.insert_inferred_types(type_vars, symbol_table, world)
            }
            Term::Case(case) => case.insert_inferred_types(type_vars, symbol_table, world),
            Term::New(new_block) => new_block.insert_inferred_types(type_vars, symbol_table, world),
            Term::Label(label) => label.insert_inferred_types(type_vars, symbol_table, world),
            Term::Goto(goto) => goto.insert_inferred_types(type_vars, symbol_table, world),
            Term::Exit(exit) => exit.insert_inferred_types(type_vars, symbol_table, world),
            Term::Paren(paren) => paren.insert_inferred_types(type_vars, symbol_table, world),
        }
    }
}

impl UsedBinders for Term {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        match self {
            Term::XVar(_) | Term::Lit(_) => {}
            Term::Op(op) => op.used_binders(used),
            Term::IfC(ifc) => ifc.used_binders(used),
            Term::PrintI64(print) => print.used_binders(used),
            Term::Let(r#let) => r#let.used_binders(used),
            Term::Call(call) => call.used_binders(used),
            Term::Constructor(constructor) => constructor.used_binders(used),
            Term::Destructor(destructor) => destructor.used_binders(used),
            Term::Case(case) => case.used_binders(used),
            Term::New(new) => new.used_binders(used),
            Term::Goto(goto) => goto.used_binders(used),
            Term::Label(label) => label.used_binders(used),
            Term::Exit(exit) => exit.used_binders(used),
            Term::Paren(paren) => paren.used_binders(used),
        }
    }
}

#[cfg(feature = "test-common")]
pub mod test_util {
    use crate::{
        syntax::{Term, TypingContext},
        typing::{
            Error,
            inference::{TypeInference, TypeInferenceState, solve_inference_constraints},
            symbol_table::SymbolTable,
        },
    };

    /// This function infers the types of a single term in the given context, like
    /// [`Program::check`](crate::syntax::program::Program::check) does for a whole program.
    pub fn infer_term(
        term: &mut Term,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
    ) -> Result<(), Error> {
        let mut state = TypeInferenceState::new(symbol_table.clone());

        let expected_ty = state.fresh_names.fresh_ty_var();
        term.collect_constraints(&mut state, context, expected_ty)?;

        let TypeInferenceState {
            mut symbol_table,
            constraints,
            choices,
            ..
        } = state;

        let (type_vars, world) = solve_inference_constraints(constraints, &choices)?;
        term.insert_inferred_types(&type_vars, &mut symbol_table, &world)
    }
}
