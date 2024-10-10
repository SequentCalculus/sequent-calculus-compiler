use crate::traits::free_vars::FreeV;
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
    Literal(Literal),
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

// Temporary, until Producers and Consumers are removed
use crate::{
    syntax,
    syntax::{Consumer, Producer},
};

impl From<Term<Prd>> for Producer {
    fn from(t: Term<Prd>) -> Producer {
        match t {
            Term::XVar(var) => Producer::Variable(syntax::Variable { var: var.var }),
            Term::Literal(lit) => Producer::Literal(syntax::Literal { lit: lit.lit }),
            Term::Mu(mu) => Producer::Mu(syntax::Mu {
                covariable: mu.variable,
                statement: mu.statement,
            }),
            Term::Xtor(xtor) => Producer::Constructor(syntax::Constructor {
                id: xtor.id,
                args: xtor.args,
            }),
            Term::XCase(xcase) => Producer::Cocase(syntax::Cocase {
                cocases: xcase.clauses,
            }),
        }
    }
}

impl From<Term<Cns>> for Consumer {
    fn from(t: Term<Cns>) -> Consumer {
        match t {
            Term::XVar(var) => Consumer::Covariable(syntax::Covariable { covar: var.var }),
            Term::Literal(_) => panic!("Cannot happen"),
            Term::Mu(mu) => Consumer::MuTilde(syntax::MuTilde {
                variable: mu.variable,
                statement: mu.statement,
            }),
            Term::Xtor(xtor) => Consumer::Destructor(syntax::Destructor {
                id: xtor.id,
                args: xtor.args,
            }),
            Term::XCase(xcase) => Consumer::Case(syntax::Case {
                cases: xcase.clauses,
            }),
        }
    }
}
