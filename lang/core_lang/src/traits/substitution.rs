//! This module defines two traits for substitution. Trait [`Subst`] provides a method for
//! substituting several variables by producers and several covariables by consumers. Trait
//! [`SubstVar`] provides a method for substituting a list of (co)variables by other (co)variables.

use crate::syntax::{
    Covar, Var,
    terms::{Cns, Prd, Term},
};

use std::rc::Rc;

/// This trait define a method for substituting several variables by producers and several
/// covariables by consumers. It also provides two methods for substituting one producer for a
/// variable or one consumer for a covariable, respectively.
pub trait Subst: Clone {
    /// The result of substituting (co)variables usually `Self::Target = Self`.
    type Target: Clone;
    /// This method substitutes several variables by producers and several covariables by consumers.
    /// It assumes that all (co)variables in terms to be substituted are fresh for the target term
    /// or statement substituted into, so care is only needed for shadowing, but not to avoid
    /// captures.
    /// - `prod_subst` is the list of producer substitutions to perform
    /// - `cons_subst` is the list of consumer substitutions to perform
    fn subst_sim(
        self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target;

    /// This method substitutes a variable by a producer. It assumes that all (co)variables in the
    /// producer to be substituted are fresh for the target term or statement substituted into, so
    /// care is only needed for shadowing, but not to avoid captures.
    fn subst_var(self, var: Var, prod: Term<Prd>) -> Self::Target {
        self.subst_sim(&[(var, prod)], &[])
    }

    /// This method substitutes a covariable by a consumer. It assumes that all (co)variables in the
    /// consumer to be substituted are fresh for the target term or statement substituted into, so
    /// care is only needed for shadowing, but not to avoid captures.
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

/// This trait defines a method for substituting a list of (co)variables for other variables.
pub trait SubstVar: Clone {
    /// The result of substitution, usually `Self::Target = Self`
    type Target;
    /// This method substitutes a list of (co)variables for other (co)variables in a term or
    /// statement. It assumes all variable bindings in each path through a term or statement to be
    /// unique, so no care is needed to account for shadowing. It further assumes that all
    /// variables substituted into the statement are fresh for this statement, so that no care is
    /// needed to avoid capture.
    /// - `subst` is the list of substitutions to perform. Each substitution is represented by a
    ///   pair with the first component being the old (co)variable substituted by the new
    ///   (co)variable in the second component.
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
