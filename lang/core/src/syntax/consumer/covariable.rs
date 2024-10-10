use super::{Consumer, Covar, Var};
use crate::{
    syntax::term::{Cns, Prd, Term},
    traits::{free_vars::FreeV, substitution::Subst},
};
use std::{collections::HashSet, fmt};

// Covariable
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Covariable {
    pub covar: Covar,
}

impl std::fmt::Display for Covariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}", self.covar)
    }
}

impl FreeV for Covariable {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        HashSet::from([self.covar.clone()])
    }
}

impl From<Covariable> for Consumer {
    fn from(value: Covariable) -> Consumer {
        Consumer::Covariable(value)
    }
}

impl Subst for Covariable {
    type Target = Consumer;

    fn subst_sim(
        &self,
        _prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        let Covariable { covar } = self;
        match cons_subst.iter().find(|(_, cv)| cv == covar) {
            None => Covariable {
                covar: covar.clone(),
            }
            .into(),
            Some((p, _)) => p.clone().into(),
        }
    }
}

#[cfg(test)]
mod covariable_tests {
    use std::collections::HashSet;

    use crate::{syntax::Covariable, traits::free_vars::FreeV};

    #[test]
    fn display() {
        let ex = Covariable {
            covar: "a".to_string(),
        };
        assert_eq!(format!("{ex}"), "'a")
    }

    #[test]
    fn free_vars() {
        let ex = Covariable {
            covar: "a".to_string(),
        };
        assert_eq!(ex.free_vars(), HashSet::new())
    }

    #[test]
    fn free_covars() {
        let ex = Covariable {
            covar: "a".to_string(),
        };
        let mut res = HashSet::new();
        res.insert("a".to_string());
        assert_eq!(ex.free_covars(), res)
    }
}
