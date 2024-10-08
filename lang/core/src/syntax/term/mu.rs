use super::{PrdCns, Term};
use crate::{
    syntax::{Covar, Statement, Var},
    traits::free_vars::FreeV,
};
use std::{collections::HashSet, fmt, rc::Rc};

// Mu
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mu<T: PrdCns> {
    pub prdcns: T,
    pub variable: Var,
    pub statement: Rc<Statement>,
}

impl<T: PrdCns> std::fmt::Display for Mu<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = if self.prdcns.is_prd() {
            "mu"
        } else {
            "mutilde"
        };
        write!(f, "{} {}. {}", prefix, self.variable, self.statement)
    }
}

impl<T: PrdCns> FreeV for Mu<T> {
    fn free_vars(&self) -> HashSet<Var> {
        let mut free_vars = FreeV::free_vars(Rc::as_ref(&self.statement));
        if self.prdcns.is_cns() {
            free_vars.remove(&self.variable);
        }
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let mut free_covars = self.statement.free_covars();
        if self.prdcns.is_prd() {
            free_covars.remove(&self.variable);
        }
        free_covars
    }
}

impl<T: PrdCns> From<Mu<T>> for Term<T> {
    fn from(value: Mu<T>) -> Self {
        Term::Mu(value)
    }
}
