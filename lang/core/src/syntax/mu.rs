use super::{Consumer, Covar, Covariable, Producer, Statement, Var};
use crate::traits::{
    free_vars::{fresh_covar, FreeV},
    substitution::Subst,
};
use std::{collections::HashSet, fmt, rc::Rc};
// Mu
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mu {
    pub covariable: Covar,
    pub statement: Rc<Statement>,
}

impl std::fmt::Display for Mu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mu {}.{}", self.covariable, self.statement)
    }
}

impl FreeV for Mu {
    fn free_vars(&self) -> HashSet<Var> {
        FreeV::free_vars(Rc::as_ref(&self.statement))
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let mut fr_cv = FreeV::free_covars(Rc::as_ref(&self.statement));
        fr_cv.remove(&self.covariable);
        fr_cv
    }
}

impl From<Mu> for Producer {
    fn from(value: Mu) -> Self {
        Producer::Mu(value)
    }
}

impl Subst for Mu {
    type Target = Mu;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        let Mu {
            covariable,
            statement,
        } = self;
        let mut fr_cv: HashSet<Covar> = statement.free_vars();
        for (cons, cv) in cons_subst.iter() {
            fr_cv.insert(cv.clone());
            fr_cv.extend(cons.free_covars());
        }
        for (prod, _) in prod_subst.iter() {
            fr_cv.extend(prod.free_covars());
        }
        let new_covar: Covar = fresh_covar(&fr_cv);
        let new_st: Rc<Statement> = statement.subst_covar(
            Covariable {
                covar: new_covar.clone(),
            }
            .into(),
            covariable.clone(),
        );
        Mu {
            covariable: new_covar,
            statement: new_st.subst_sim(prod_subst, cons_subst),
        }
    }
}

#[cfg(test)]
mod mu_tests {
    use std::rc::Rc;

    use crate::syntax::Mu;

    use super::Statement;

    #[test]
    fn display() {
        let ex = Mu {
            covariable: "a".to_string(),
            statement: Rc::new(Statement::Done()),
        };
        assert_eq!(format!("{ex}"), "mu a.Done".to_string())
    }
}
