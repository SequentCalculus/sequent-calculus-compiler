use printer::Print;

use crate::{
    syntax::{Covar, FsStatement, Ty, Var},
    traits::*,
};

use std::collections::HashSet;

mod literal;
mod mu;
mod xcase;
mod xtor;
mod xvar;

pub use literal::Literal;
pub use mu::{FsMu, Mu};
pub use xcase::{Clause, XCase};
pub use xtor::{FsXtor, Xtor};
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term<T: PrdCns> {
    XVar(XVar<T>),
    Literal(Literal),
    Mu(Mu<T>),
    Xtor(Xtor<T>),
    XCase(XCase<T, Statement>),
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

impl<T: PrdCns> Print for Term<T> {
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

impl<T: PrdCns> FreeV for Term<T> {
    fn free_vars(self: &Term<T>) -> HashSet<Var> {
        match self {
            Term::XVar(var) => var.free_vars(),
            Term::Literal(lit) => lit.free_vars(),
            Term::Mu(mu) => mu.free_vars(),
            Term::Xtor(xtor) => xtor.free_vars(),
            Term::XCase(xcase) => xcase.free_vars(),
        }
    }

    fn free_covars(self: &Term<T>) -> HashSet<Covar> {
        match self {
            Term::XVar(var) => var.free_covars(),
            Term::Literal(lit) => lit.free_covars(),
            Term::Mu(mu) => mu.free_covars(),
            Term::Xtor(xtor) => xtor.free_covars(),
            Term::XCase(xcase) => xcase.free_covars(),
        }
    }
}

impl<T: PrdCns> UsedBinders for Term<T> {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        match self {
            Term::Mu(mu) => mu.used_binders(used),
            Term::XCase(xtor) => xtor.used_binders(used),
            Term::Xtor(xcase) => xcase.used_binders(used),
            _ => {}
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

impl<T: PrdCns> Uniquify for Term<T> {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Term<T> {
        match self {
            Term::Mu(mu) => mu.uniquify(seen_vars, used_vars).into(),
            Term::Xtor(xtor) => xtor.uniquify(seen_vars, used_vars).into(),
            Term::XCase(xcase) => xcase.uniquify(seen_vars, used_vars).into(),
            _ => self,
        }
    }
}

impl Focusing for Term<Prd> {
    type Target = FsTerm<Prd>;

    fn focus(self, st: &mut FocusingState) -> Self::Target {
        match self {
            Term::XVar(var) => var.into(),
            Term::Literal(lit) => lit.into(),
            Term::Mu(mu) => mu.focus(st).into(),
            Term::Xtor(xtor) => xtor.focus(st),
            Term::XCase(xcase) => xcase.focus(st).into(),
        }
    }
}
impl Focusing for Term<Cns> {
    type Target = FsTerm<Cns>;

    fn focus(self, st: &mut FocusingState) -> Self::Target {
        match self {
            Term::XVar(covar) => covar.into(),
            Term::Literal(_) => panic!("Cannot happen"),
            Term::Mu(mu) => mu.focus(st).into(),
            Term::Xtor(xtor) => xtor.focus(st),
            Term::XCase(xcase) => xcase.focus(st).into(),
        }
    }
}

impl Bind for Term<Prd> {
    fn bind(self, k: Continuation, state: &mut FocusingState) -> FsStatement {
        match self {
            Term::XVar(var) => var.bind(k, state),
            Term::Literal(lit) => lit.bind(k, state),
            Term::Mu(mu) => mu.bind(k, state),
            Term::Xtor(xtor) => xtor.bind(k, state),
            Term::XCase(xcase) => xcase.bind(k, state),
        }
    }
}
impl Bind for Term<Cns> {
    fn bind(self, k: Continuation, state: &mut FocusingState) -> FsStatement {
        match self {
            Term::XVar(covar) => covar.bind(k, state),
            Term::Literal(lit) => lit.bind(k, state),
            Term::Mu(mu) => mu.bind(k, state),
            Term::Xtor(xtor) => xtor.bind(k, state),
            Term::XCase(xcase) => xcase.bind(k, state),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsTerm<T: PrdCns> {
    XVar(XVar<T>),
    Literal(Literal),
    Mu(FsMu<T>),
    Xtor(FsXtor<T>),
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
            FsTerm::Literal(lit) => FsTerm::Literal(lit),
            FsTerm::Mu(mu) => mu.subst_sim(subst).into(),
            FsTerm::Xtor(xtor) => xtor.subst_sim(subst).into(),
            FsTerm::XCase(xcase) => xcase.subst_sim(subst).into(),
        }
    }
}
