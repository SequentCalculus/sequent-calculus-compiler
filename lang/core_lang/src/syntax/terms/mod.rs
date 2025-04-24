use printer::Print;

use crate::{
    syntax::{ContextBinding, Covar, FsStatement, Ty, Var},
    traits::*,
};

use std::collections::{BTreeSet, HashSet};

mod clause;
mod literal;
mod mu;
mod xcase;
mod xtor;
mod xvar;

pub use clause::{Clause, print_clauses};
pub use literal::Literal;
pub use mu::Mu;
pub use xcase::XCase;
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
    Mu(Mu<T, Statement>),
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
            Term::Literal(_) => panic!("cannot happen"),
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
            Term::Mu(mu) => mu.typed_free_vars(vars),
            Term::Xtor(xtor) => xtor.typed_free_vars(vars),
            Term::XCase(xcase) => xcase.typed_free_vars(vars),
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
    fn focus(self, used_vars: &mut HashSet<Var>) -> Self::Target {
        match self {
            Term::XVar(var) => var.into(),
            Term::Literal(lit) => lit.into(),
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
            Term::Literal(_) => panic!("Cannot happen"),
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
            Term::Literal(lit) => lit.bind(k, used_vars),
            Term::Mu(mu) => mu.bind(k, used_vars),
            Term::Xtor(xtor) => xtor.bind(k, used_vars),
            Term::XCase(xcase) => xcase.bind(k, used_vars),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsTerm<T: PrdCns> {
    XVar(XVar<T>),
    Literal(Literal),
    Mu(Mu<T, FsStatement>),
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
            FsTerm::Literal(ref _lit) => self,
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
            FsTerm::Mu(mu) => mu.typed_free_vars(vars),
            FsTerm::Xtor(xtor) => xtor.typed_free_vars(vars),
            FsTerm::XCase(xcase) => xcase.typed_free_vars(vars),
        }
    }
}
