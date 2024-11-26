use mu::FsMu;
use printer::Print;
use xcase::FsXCase;
use xtor::FsXtor;
use xvar::FsXVar;

use crate::{
    syntax::{
        types::{Ty, Typed},
        Covar, Var,
    },
    traits::{
        focus::{Bind, Continuation, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::{Subst, SubstVar},
        uniquify::Uniquify,
        used_binders::UsedBinders,
    },
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
    fn free_vars(self: &Term<T>) -> HashSet<crate::syntax::Var> {
        match self {
            Term::XVar(var) => var.free_vars(),
            Term::Literal(lit) => lit.free_vars(),
            Term::Mu(mu) => mu.free_vars(),
            Term::Xtor(xtor) => xtor.free_vars(),
            Term::XCase(xcase) => xcase.free_vars(),
        }
    }

    fn free_covars(self: &Term<T>) -> HashSet<crate::syntax::Covar> {
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
    type Target = FsTerm;

    fn focus(self, st: &mut FocusingState) -> Self::Target {
        match self {
            Term::XVar(var) => var.focus(st).into(),
            Term::Literal(lit) => lit.into(),
            Term::Mu(mu) => mu.focus(st).into(),
            Term::Xtor(xtor) => xtor.focus(st),
            Term::XCase(xcase) => xcase.focus(st).into(),
        }
    }
}
impl Focusing for Term<Cns> {
    type Target = FsTerm;

    fn focus(self, st: &mut FocusingState) -> Self::Target {
        match self {
            Term::XVar(covar) => covar.focus(st).into(),
            Term::Literal(_) => panic!("Cannot happen"),
            Term::Mu(mu) => mu.focus(st).into(),
            Term::Xtor(xtor) => xtor.focus(st),
            Term::XCase(xcase) => xcase.focus(st).into(),
        }
    }
}

impl Bind for Term<Prd> {
    fn bind(
        self,
        k: Continuation,
        state: &mut FocusingState,
    ) -> crate::syntax::statement::FsStatement {
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
    fn bind(
        self,
        k: Continuation,
        state: &mut FocusingState,
    ) -> crate::syntax::statement::FsStatement {
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
pub enum FsTerm {
    XVar(FsXVar),
    Literal(Literal),
    Mu(FsMu),
    Xtor(FsXtor),
    XCase(FsXCase),
}

impl Print for FsTerm {
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

impl SubstVar for FsTerm {
    type Target = FsTerm;
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
