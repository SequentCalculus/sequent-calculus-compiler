use crate::traits::{free_vars::FreeV, substitution::Subst};
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

pub struct Prd;
pub struct Cns;

pub trait PrdCns {
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
    Literal(Literal<T>),
    Mu(Mu<T>),
    Xtor(Xtor<T>),
    XCase(XCase<T>),
}

impl std::fmt::Display for Term<Prd> {
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
impl std::fmt::Display for Term<Cns> {
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

impl FreeV for Term<Prd> {
    fn free_vars(self: &Term<Prd>) -> HashSet<crate::syntax::Var> {
        match self {
            Term::XVar(v) => v.free_vars(),
            Term::Literal(l) => l.free_vars(),
            Term::Mu(m) => m.free_vars(),
            Term::Xtor(c) => c.free_vars(),
            Term::XCase(c) => c.free_vars(),
        }
    }

    fn free_covars(self: &Term<Prd>) -> HashSet<crate::syntax::Covar> {
        match self {
            Term::XVar(v) => v.free_covars(),
            Term::Literal(l) => l.free_covars(),
            Term::Mu(m) => m.free_covars(),
            Term::Xtor(c) => c.free_covars(),
            Term::XCase(c) => c.free_covars(),
        }
    }
}
impl FreeV for Term<Cns> {
    fn free_vars(self: &Term<Cns>) -> HashSet<crate::syntax::Var> {
        match self {
            Term::XVar(v) => v.free_vars(),
            Term::Literal(l) => l.free_vars(),
            Term::Mu(m) => m.free_vars(),
            Term::Xtor(c) => c.free_vars(),
            Term::XCase(c) => c.free_vars(),
        }
    }

    fn free_covars(self: &Term<Cns>) -> HashSet<crate::syntax::Covar> {
        match self {
            Term::XVar(v) => v.free_covars(),
            Term::Literal(l) => l.free_covars(),
            Term::Mu(m) => m.free_covars(),
            Term::Xtor(c) => c.free_covars(),
            Term::XCase(c) => c.free_covars(),
        }
    }
}

impl Subst for Producer {
    type Target = Producer;
    fn subst_sim(
        self: &Producer,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Producer {
        match self {
            Producer::XVar(v) => v.subst_sim(prod_subst, cons_subst),
            Producer::Literal(l) => l.subst_sim(prod_subst, cons_subst).into(),
            Producer::Mu(m) => m.subst_sim(prod_subst, cons_subst).into(),
            Producer::Xtor(c) => c.subst_sim(prod_subst, cons_subst).into(),
            Producer::XCase(c) => c.subst_sim(prod_subst, cons_subst).into(),
        }
    }
}
