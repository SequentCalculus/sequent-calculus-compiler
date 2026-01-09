//! This module defines the terms of Fun.

pub mod call;
pub mod call_template;
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
pub use call_template::*;
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
    traits::used_binders::UsedBinders,
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use super::{
    context::TypingContext,
    types::{OptTyped, Ty},
};

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
    /// Call of a top-level template
    CallTemplate(CallTemplate),
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
            Term::CallTemplate(ct) => ct.get_type(),
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
            Term::CallTemplate(ct) => ct.print(cfg, alloc),
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

impl Check for Term {
    fn check(
        self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        match self {
            Term::XVar(var) => var.check(symbol_table, context, expected).map(Into::into),
            Term::Lit(lit) => lit.check(symbol_table, context, expected).map(Into::into),
            Term::Op(op) => op.check(symbol_table, context, expected).map(Into::into),
            Term::IfC(ifc) => ifc.check(symbol_table, context, expected).map(Into::into),
            Term::PrintI64(print) => print.check(symbol_table, context, expected).map(Into::into),
            Term::Let(r#letxp) => r#letxp
                .check(symbol_table, context, expected)
                .map(Into::into),
            Term::Call(call) => call.check(symbol_table, context, expected).map(Into::into),
            Term::CallTemplate(ct) => ct.check(symbol_table, context, expected).map(Into::into),
            Term::Constructor(constructor) => constructor
                .check(symbol_table, context, expected)
                .map(Into::into),
            Term::Destructor(destructor) => destructor
                .check(symbol_table, context, expected)
                .map(Into::into),
            Term::Case(case) => case.check(symbol_table, context, expected).map(Into::into),
            Term::New(new) => new.check(symbol_table, context, expected).map(Into::into),
            Term::Goto(goto) => goto.check(symbol_table, context, expected).map(Into::into),
            Term::Label(label) => label.check(symbol_table, context, expected).map(Into::into),
            Term::Exit(exit) => exit.check(symbol_table, context, expected).map(Into::into),
            Term::Paren(paren) => paren.check(symbol_table, context, expected).map(Into::into),
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
            Term::CallTemplate(ct) => ct.used_binders(used),
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
