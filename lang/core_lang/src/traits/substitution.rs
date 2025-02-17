use crate::syntax::{
    terms::{Cns, Prd, Term},
    Covar, Var,
};

use std::rc::Rc;

/// Assumes all variables in terms to be substituted are fresh for the target terms substituted
/// into, so care is only needed for shadowing, but not to avoid captures.
pub trait Subst: Clone {
    type Target: Clone;
    fn subst_sim(
        self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target;

    fn subst_var(self, var: Var, prod: Term<Prd>) -> Self::Target {
        self.subst_sim(&[(var, prod)], &[])
    }
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

impl<T: Subst + Clone> Subst for Vec<T> {
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

/// Assumes all variable bindings to be unique, so no care is needed to avoid captures or
/// shadowing.
pub trait SubstVar: Clone {
    type Target;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target;
}

impl<T: SubstVar> SubstVar for Rc<T> {
    type Target = Rc<T::Target>;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).subst_sim(subst))
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
