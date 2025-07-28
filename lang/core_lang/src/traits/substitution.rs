//! Defines the [Subst]-trait for substituting variables and covariables
use crate::syntax::{
    Covar, Var,
    terms::{Cns, Prd, Term},
};

use std::rc::Rc;

/// Substitute Variables by Producers and Covariables by Consumers
/// Assumes all variables in terms to be substituted are fresh for the target terms substituted
/// into, so care is only needed for shadowing, but not to avoid captures.
pub trait Subst: Clone {
    /// The result of substituting (co-) variables
    /// usually `Self::Target = Self`
    type Target: Clone;
    /// Substitute all variables in `prod_subst` by their corresponding producer terms
    /// and substitute all covariables in `cons_subst` by their corresponding consumer terms
    fn subst_sim(
        self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target;

    /// Substitute a variable by a given producer term
    fn subst_var(self, var: Var, prod: Term<Prd>) -> Self::Target {
        self.subst_sim(&[(var, prod)], &[])
    }

    /// Substitute a covariable by a given consumer term
    fn subst_covar(self, covar: Covar, cons: Term<Cns>) -> Self::Target {
        self.subst_sim(&[], &[(covar, cons)])
    }
}

impl<T: Subst> Subst for Rc<T> {
    type Target = Rc<T::Target>;
    fn subst_sim(
        self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).subst_sim(prod_subst, cons_subst))
    }
}

impl<T: Subst> Subst for Option<T> {
    type Target = Option<T::Target>;
    fn subst_sim(
        self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Option<T::Target> {
        self.map(|t| t.subst_sim(prod_subst, cons_subst))
    }
}

impl<T: Subst> Subst for Vec<T> {
    type Target = Vec<T::Target>;
    fn subst_sim(
        self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Vec<T::Target> {
        self.into_iter()
            .map(|element| element.subst_sim(prod_subst, cons_subst))
            .collect()
    }
}

/// Substitute Variables by other variables
/// In other words, rename variables
/// Assumes all variable bindings to be unique, so no care is needed to avoid captures or
/// shadowing.
pub trait SubstVar: Clone {
    /// The result of substitution
    /// Usually `Self::Target = Self`
    type Target;
    /// Substitute all variables in the given list by their new names
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target;
}

impl<T: SubstVar> SubstVar for Rc<T> {
    type Target = Rc<T::Target>;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).subst_sim(subst))
    }
}

impl<T: SubstVar> SubstVar for Option<T> {
    type Target = Option<T::Target>;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Option<T::Target> {
        self.map(|t| t.subst_sim(subst))
    }
}

impl<T: SubstVar> SubstVar for Vec<T> {
    type Target = Vec<T::Target>;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Vec<T::Target> {
        self.into_iter()
            .map(|element| element.subst_sim(subst))
            .collect()
    }
}
