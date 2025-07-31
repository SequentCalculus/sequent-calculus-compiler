//! This module defines the terms (producers and consumers) of Core.

use printer::Print;

use crate::{
    syntax::{ContextBinding, Covar, FsStatement, Ty, Var},
    traits::*,
};

use std::collections::{BTreeSet, HashSet};

pub mod clause;
pub mod literal;
pub mod mu;
pub mod op;
pub mod xcase;
pub mod xtor;
pub mod xvar;

pub use clause::{Clause, print_clauses};
pub use literal::Literal;
pub use mu::Mu;
pub use op::{BinOp, FsOp, Op};
pub use xcase::XCase;
pub use xtor::{FsXtor, Xtor};
pub use xvar::XVar;

use super::Statement;

/// This marker trait allows to abstract over the information of whether something is a producer or
/// a consumer.
pub trait PrdCns: Clone {
    /// This method returns whether something is a producer.
    fn is_prd(&self) -> bool;
    /// This method returns whether something is a consumer.
    fn is_cns(&self) -> bool {
        !self.is_prd()
    }
}

/// This marker struct is used to instantiate a type parameter satisfying the [PrdCns] marker trait
/// as producer.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Prd;
/// This marker struct is used to instantiate a type parameter satisfying the [PrdCns] marker trait
/// as consumer.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cns;

impl PrdCns for Prd {
    fn is_prd(&self) -> bool {
        true
    }
}

impl PrdCns for Cns {
    fn is_prd(&self) -> bool {
        false
    }
}

/// This enum defines the terms of Core. It contains one variant for each construct which simply
/// wraps the struct defining the corresponding construct.  The type parameter `T` determines for
/// some of the variants whether they are a producer (if `T` is instantiated with [`Prd`]) or a
/// consumer (if `T` is instantiated with [`Cns`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term<T: PrdCns> {
    /// Variable or covariable
    XVar(XVar<T>),
    /// Integer literal (always producer)
    Literal(Literal),
    /// Arithmetic binary operations (always producer)
    Op(Op),
    /// `mu`- or `mu-tilde` binding
    Mu(Mu<T, Statement>),
    /// Constructor or destructor
    Xtor(Xtor<T>),
    /// Pattern match or Copattern match
    XCase(XCase<T, Statement>),
}

impl<T: PrdCns> Typed for Term<T> {
    fn get_type(&self) -> Ty {
        match self {
            Term::XVar(var) => var.get_type(),
            Term::Literal(lit) => lit.get_type(),
            Term::Op(op) => op.get_type(),
            Term::Mu(mu) => mu.get_type(),
            Term::Xtor(xtor) => xtor.get_type(),
            Term::XCase(xcase) => xcase.get_type(),
        }
    }
}

impl<T: PrdCns> Print for Term<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Term::XVar(var) => var.print(cfg, alloc),
            Term::Literal(lit) => lit.print(cfg, alloc),
            Term::Op(op) => op.print(cfg, alloc),
            Term::Mu(mu) => mu.print(cfg, alloc),
            Term::Xtor(xtor) => xtor.print(cfg, alloc),
            Term::XCase(xcase) => xcase.print(cfg, alloc),
        }
    }
}

impl Subst for Term<Prd> {
    type Target = Term<Prd>;
    fn subst_sim(
        self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        match self {
            Term::XVar(var) => Subst::subst_sim(var, prod_subst, cons_subst),
            Term::Literal(ref _lit) => self,
            Term::Op(op) => op.subst_sim(prod_subst, cons_subst).into(),
            Term::Mu(mu) => mu.subst_sim(prod_subst, cons_subst).into(),
            Term::Xtor(xtor) => xtor.subst_sim(prod_subst, cons_subst).into(),
            Term::XCase(xcase) => xcase.subst_sim(prod_subst, cons_subst).into(),
        }
    }
}
impl Subst for Term<Cns> {
    type Target = Term<Cns>;
    fn subst_sim(
        self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        match self {
            Term::XVar(var) => Subst::subst_sim(var, prod_subst, cons_subst),
            Term::Literal(_) | Term::Op(_) => panic!("cannot happen"),
            Term::Mu(mu) => mu.subst_sim(prod_subst, cons_subst).into(),
            Term::Xtor(xtor) => xtor.subst_sim(prod_subst, cons_subst).into(),
            Term::XCase(xcase) => xcase.subst_sim(prod_subst, cons_subst).into(),
        }
    }
}

impl<T: PrdCns> TypedFreeVars for Term<T> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        match self {
            Term::XVar(var) => var.typed_free_vars(vars),
            Term::Literal(_) => {}
            Term::Op(op) => op.typed_free_vars(vars),
            Term::Mu(mu) => mu.typed_free_vars(vars),
            Term::Xtor(xtor) => xtor.typed_free_vars(vars),
            Term::XCase(xcase) => xcase.typed_free_vars(vars),
        }
    }
}

impl<T: PrdCns> Uniquify for Term<T> {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Term<T> {
        match self {
            Term::Op(op) => op.uniquify(seen_vars, used_vars).into(),
            Term::Mu(mu) => mu.uniquify(seen_vars, used_vars).into(),
            Term::Xtor(xtor) => xtor.uniquify(seen_vars, used_vars).into(),
            Term::XCase(xcase) => xcase.uniquify(seen_vars, used_vars).into(),
            _ => self,
        }
    }
}

impl Focusing for Term<Prd> {
    type Target = FsTerm<Prd>;
    fn focus(self, used_vars: &mut HashSet<Var>) -> Self::Target {
        match self {
            Term::XVar(var) => var.into(),
            Term::Literal(lit) => lit.into(),
            Term::Op(op) => op.focus(used_vars),
            Term::Mu(mu) => mu.focus(used_vars).into(),
            Term::Xtor(xtor) => xtor.focus(used_vars),
            Term::XCase(xcase) => xcase.focus(used_vars).into(),
        }
    }
}
impl Focusing for Term<Cns> {
    type Target = FsTerm<Cns>;
    fn focus(self, used_vars: &mut HashSet<Var>) -> Self::Target {
        match self {
            Term::XVar(covar) => covar.into(),
            Term::Literal(_) | Term::Op(_) => panic!("Cannot happen"),
            Term::Mu(mu) => mu.focus(used_vars).into(),
            Term::Xtor(xtor) => xtor.focus(used_vars),
            Term::XCase(xcase) => xcase.focus(used_vars).into(),
        }
    }
}

impl Bind for Term<Prd> {
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        match self {
            Term::XVar(var) => var.bind(k, used_vars),
            Term::Literal(lit) => lit.bind(k, used_vars),
            Term::Op(op) => op.bind(k, used_vars),
            Term::Mu(mu) => mu.bind(k, used_vars),
            Term::Xtor(xtor) => xtor.bind(k, used_vars),
            Term::XCase(xcase) => xcase.bind(k, used_vars),
        }
    }
}
impl Bind for Term<Cns> {
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        match self {
            Term::XVar(covar) => covar.bind(k, used_vars),
            Term::Literal(_) | Term::Op(_) => panic!("Cannot happen"),
            Term::Mu(mu) => mu.bind(k, used_vars),
            Term::Xtor(xtor) => xtor.bind(k, used_vars),
            Term::XCase(xcase) => xcase.bind(k, used_vars),
        }
    }
}

/// This struct defines the focused version of [`Term`]s. In focused terms only (co)variables can
/// occur in argument positions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsTerm<T: PrdCns> {
    /// Variable or covariable
    XVar(XVar<T>),
    /// Integer literal (always producer)
    Literal(Literal),
    /// Arithmetic binary operations (always producer)
    Op(FsOp),
    /// `mu`- or `mu-tilde` binding
    Mu(Mu<T, FsStatement>),
    /// Constructor or destructor
    Xtor(FsXtor<T>),
    /// Pattern match or Copattern match
    XCase(XCase<T, FsStatement>),
}

impl<T: PrdCns> Print for FsTerm<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            FsTerm::XVar(var) => var.print(cfg, alloc),
            FsTerm::Literal(lit) => lit.print(cfg, alloc),
            FsTerm::Op(op) => op.print(cfg, alloc),
            FsTerm::Mu(mu) => mu.print(cfg, alloc),
            FsTerm::Xtor(xtor) => xtor.print(cfg, alloc),
            FsTerm::XCase(xcase) => xcase.print(cfg, alloc),
        }
    }
}

impl<T: PrdCns> SubstVar for FsTerm<T> {
    type Target = FsTerm<T>;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        match self {
            FsTerm::XVar(var) => var.subst_sim(subst).into(),
            FsTerm::Literal(ref _lit) => self,
            FsTerm::Op(op) => op.subst_sim(subst).into(),
            FsTerm::Mu(mu) => mu.subst_sim(subst).into(),
            FsTerm::Xtor(xtor) => xtor.subst_sim(subst).into(),
            FsTerm::XCase(xcase) => xcase.subst_sim(subst).into(),
        }
    }
}

impl<T: PrdCns> TypedFreeVars for FsTerm<T> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        match self {
            FsTerm::XVar(var) => var.typed_free_vars(vars),
            FsTerm::Literal(_) => {}
            FsTerm::Op(op) => op.typed_free_vars(vars),
            FsTerm::Mu(mu) => mu.typed_free_vars(vars),
            FsTerm::Xtor(xtor) => xtor.typed_free_vars(vars),
            FsTerm::XCase(xcase) => xcase.typed_free_vars(vars),
        }
    }
}
