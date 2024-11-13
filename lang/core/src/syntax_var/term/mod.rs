use crate::{syntax_var::Var, traits::substitution::SubstVar};

use std::fmt;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    XVar(XVar),
    Literal(Literal),
    Mu(Mu),
    Xtor(Xtor),
    XCase(XCase),
}

impl std::fmt::Display for Term {
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

impl SubstVar for Term {
    type Target = Term;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        match self {
            Term::XVar(var) => var.subst_sim(subst).into(),
            Term::Literal(lit) => Term::Literal(lit),
            Term::Mu(mu) => mu.subst_sim(subst).into(),
            Term::Xtor(xtor) => xtor.subst_sim(subst).into(),
            Term::XCase(xcase) => xcase.subst_sim(subst).into(),
        }
    }
}
