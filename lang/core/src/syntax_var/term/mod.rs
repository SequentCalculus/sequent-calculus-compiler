use printer::Print;

use crate::{
    syntax_var::Var,
    traits::{substitution::SubstVar, used_binders::UsedBinders},
};

use std::collections::HashSet;

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

impl Print for Term {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Term::XVar(var) => var.print(cfg, alloc),
            Term::Literal(lit) => lit.print(cfg, alloc),
            Term::Mu(mu) => mu.print(cfg, alloc),
            Term::Xtor(xtor) => xtor.print(cfg, alloc),
            Term::XCase(xcase) => xcase.print(cfg, alloc),
        }
    }
}

impl UsedBinders for Term {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        match self {
            Term::Mu(mu) => mu.used_binders(used),
            Term::XCase(xcase) => xcase.used_binders(used),
            _ => {}
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
