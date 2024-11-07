use crate::{
    syntax::{types::Ty, Covar, Var},
    traits::{
        focus::{Bind, Continuation, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
        typed::Typed,
    },
};
use std::{collections::HashSet, fmt};

pub mod literal;
pub mod mu;
pub mod xcase;
pub mod xtor;
pub mod xvar;
pub use literal::Literal;
pub use mu::Mu;
pub use xcase::XCase;
pub use xtor::Xtor;
pub use xvar::XVar;

use super::Statement;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Prd;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cns;

pub trait PrdCns: Clone {
    fn is_prd(&self) -> bool;
    fn is_cns(&self) -> bool {
        !self.is_prd()
    }
}

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

// Term
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term<T: PrdCns> {
    XVar(XVar<T>),
    Literal(Literal),
    Mu(Mu<T>),
    Xtor(Xtor<T>),
    XCase(XCase<T>),
}

impl<T: PrdCns> Typed for Term<T> {
    fn get_type(&self) -> Ty {
        match self {
            Term::XVar(var) => var.get_type(),
            Term::Literal(lit) => lit.get_type(),
            Term::Mu(mu) => mu.get_type(),
            Term::Xtor(xtor) => xtor.get_type(),
            Term::XCase(xcase) => xcase.get_type(),
        }
    }
}

impl<T: PrdCns> std::fmt::Display for Term<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::XVar(v) => v.fmt(f),
            Term::Literal(i) => i.fmt(f),
            Term::Mu(m) => m.fmt(f),
            Term::Xtor(c) => c.fmt(f),
            Term::XCase(c) => c.fmt(f),
        }
    }
}

impl<T: PrdCns> FreeV for Term<T> {
    fn free_vars(self: &Term<T>) -> HashSet<crate::syntax::Var> {
        match self {
            Term::XVar(v) => v.free_vars(),
            Term::Literal(l) => l.free_vars(),
            Term::Mu(m) => m.free_vars(),
            Term::Xtor(c) => c.free_vars(),
            Term::XCase(c) => c.free_vars(),
        }
    }

    fn free_covars(self: &Term<T>) -> HashSet<crate::syntax::Covar> {
        match self {
            Term::XVar(v) => v.free_covars(),
            Term::Literal(l) => l.free_covars(),
            Term::Mu(m) => m.free_covars(),
            Term::Xtor(c) => c.free_covars(),
            Term::XCase(c) => c.free_covars(),
        }
    }
}

impl Subst for Term<Prd> {
    type Target = Term<Prd>;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        match self {
            Term::XVar(var) => var.subst_sim(prod_subst, cons_subst),
            Term::Literal(lit) => lit.subst_sim(prod_subst, cons_subst).into(),
            Term::Mu(mu) => mu.subst_sim(prod_subst, cons_subst).into(),
            Term::Xtor(xtor) => xtor.subst_sim(prod_subst, cons_subst).into(),
            Term::XCase(xcase) => xcase.subst_sim(prod_subst, cons_subst).into(),
        }
    }
}
impl Subst for Term<Cns> {
    type Target = Term<Cns>;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        match self {
            Term::XVar(var) => var.subst_sim(prod_subst, cons_subst),
            Term::Literal(_) => panic!("cannot happen"),
            Term::Mu(mu) => mu.subst_sim(prod_subst, cons_subst).into(),
            Term::Xtor(xtor) => xtor.subst_sim(prod_subst, cons_subst).into(),
            Term::XCase(xcase) => xcase.subst_sim(prod_subst, cons_subst).into(),
        }
    }
}

impl Focusing for Term<Prd> {
    type Target = Term<Prd>;

    fn focus(self, st: &mut FocusingState) -> Self::Target {
        match self {
            Term::XVar(var) => Term::XVar(var),
            Term::Literal(lit) => Term::Literal(lit),
            Term::Mu(mu) => mu.focus(st).into(),
            Term::Xtor(xtor) => xtor.focus(st),
            Term::XCase(xcase) => xcase.focus(st).into(),
        }
    }
}

impl Focusing for Term<Cns> {
    type Target = Term<Cns>;

    fn focus(self, st: &mut FocusingState) -> Self::Target {
        match self {
            Term::XVar(var) => Term::XVar(var),
            Term::Literal(lit) => Term::Literal(lit),
            Term::Mu(mu) => mu.focus(st).into(),
            Term::Xtor(xtor) => xtor.focus(st),
            Term::XCase(xcase) => xcase.focus(st).into(),
        }
    }
}

impl Bind for Term<Prd> {
    fn bind(self, k: Continuation, st: &mut FocusingState) -> Statement {
        match self {
            Term::XVar(xvar) => k(xvar.var, st),
            Term::Literal(lit) => lit.bind(k, st),
            Term::Mu(mu) => mu.bind(k, st),
            Term::Xtor(xtor) => xtor.bind(k, st),
            Term::XCase(xcase) => xcase.bind(k, st),
        }
    }
}

impl Bind for Term<Cns> {
    fn bind(self, k: Continuation, st: &mut FocusingState) -> Statement {
        match self {
            Term::XVar(xvar) => k(xvar.var, st),
            Term::Literal(lit) => lit.bind(k, st),
            Term::Mu(mu) => mu.bind(k, st),
            Term::Xtor(xtor) => xtor.bind(k, st),
            Term::XCase(xcase) => xcase.bind(k, st),
        }
    }
}
