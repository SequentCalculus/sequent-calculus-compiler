use printer::Print;

mod co_case;
mod constructor;
mod destructor;
mod fun;
mod goto;
mod ifc;
mod ifz;
mod label;
mod lit;
mod local_let;
mod op;
mod paren;
mod var;

pub use co_case::*;
pub use constructor::*;
pub use destructor::*;
pub use fun::*;
pub use goto::*;
pub use ifc::*;
pub use ifz::*;
pub use label::*;
pub use lit::*;
pub use local_let::*;
pub use op::*;
pub use paren::*;
pub use var::*;

use crate::{
    syntax::XVar,
    traits::UsedBinders,
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use super::{
    context::TypingContext,
    types::{OptTyped, Ty},
};

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Var),
    Lit(Lit),
    Op(Op),
    IfC(IfC),
    IfZ(IfZ),
    Let(Let),
    Fun(Fun),
    Constructor(Constructor),
    Destructor(Destructor),
    Case(Case),
    Cocase(Cocase),
    Goto(Goto),
    Label(Label),
    Paren(Paren),
}

impl OptTyped for Term {
    fn get_type(&self) -> Option<Ty> {
        match self {
            Term::Var(var) => var.get_type(),
            Term::Lit(lit) => lit.get_type(),
            Term::Op(op) => op.get_type(),
            Term::IfC(ifc) => ifc.get_type(),
            Term::IfZ(ifz) => ifz.get_type(),
            Term::Let(lt) => lt.get_type(),
            Term::Fun(fun) => fun.get_type(),
            Term::Constructor(ctor) => ctor.get_type(),
            Term::Destructor(dtor) => dtor.get_type(),
            Term::Case(case) => case.get_type(),
            Term::Cocase(cocase) => cocase.get_type(),
            Term::Goto(goto) => goto.get_type(),
            Term::Label(lb) => lb.get_type(),
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
            Term::Var(var) => var.print(cfg, alloc),
            Term::Lit(lit) => lit.print(cfg, alloc),
            Term::Op(op) => op.print(cfg, alloc),
            Term::IfC(ifc) => ifc.print(cfg, alloc),
            Term::IfZ(ifz) => ifz.print(cfg, alloc),
            Term::Let(lete) => lete.print(cfg, alloc),
            Term::Fun(fun) => fun.print(cfg, alloc),
            Term::Constructor(constructor) => constructor.print(cfg, alloc),
            Term::Destructor(destructor) => destructor.print(cfg, alloc),
            Term::Case(case) => case.print(cfg, alloc),
            Term::Cocase(cocase) => cocase.print(cfg, alloc),
            Term::Goto(goto) => goto.print(cfg, alloc),
            Term::Label(label) => label.print(cfg, alloc),
            Term::Paren(paren) => paren.print(cfg, alloc),
        }
    }
}

impl Check for Term {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        match self {
            Term::Var(var) => var.check(symbol_table, context, expected).map(Into::into),
            Term::Lit(lit) => lit.check(symbol_table, context, expected).map(Into::into),
            Term::Op(op) => op.check(symbol_table, context, expected).map(Into::into),
            Term::IfC(ifc) => ifc.check(symbol_table, context, expected).map(Into::into),
            Term::IfZ(ifz) => ifz.check(symbol_table, context, expected).map(Into::into),
            Term::Let(letexp) => letexp
                .check(symbol_table, context, expected)
                .map(Into::into),
            Term::Fun(fun) => fun.check(symbol_table, context, expected).map(Into::into),
            Term::Constructor(constructor) => constructor
                .check(symbol_table, context, expected)
                .map(Into::into),
            Term::Destructor(destructor) => destructor
                .check(symbol_table, context, expected)
                .map(Into::into),
            Term::Case(case) => case.check(symbol_table, context, expected).map(Into::into),
            Term::Cocase(cocase) => cocase
                .check(symbol_table, context, expected)
                .map(Into::into),
            Term::Goto(goto) => goto.check(symbol_table, context, expected).map(Into::into),
            Term::Label(label) => label.check(symbol_table, context, expected).map(Into::into),
            Term::Paren(paren) => paren.check(symbol_table, context, expected).map(Into::into),
        }
    }
}

impl UsedBinders for Term {
    fn used_binders(&self, used: &mut HashSet<XVar>) {
        match self {
            Term::Var(_) | Term::Lit(_) => {}
            Term::Op(op) => op.used_binders(used),
            Term::IfC(ifc) => ifc.used_binders(used),
            Term::IfZ(ifz) => ifz.used_binders(used),
            Term::Let(lete) => lete.used_binders(used),
            Term::Fun(fun) => fun.used_binders(used),
            Term::Constructor(constructor) => constructor.used_binders(used),
            Term::Destructor(destructor) => destructor.used_binders(used),
            Term::Case(case) => case.used_binders(used),
            Term::Cocase(cocase) => cocase.used_binders(used),
            Term::Goto(goto) => goto.used_binders(used),
            Term::Label(label) => label.used_binders(used),
            Term::Paren(paren) => paren.used_binders(used),
        }
    }
}
