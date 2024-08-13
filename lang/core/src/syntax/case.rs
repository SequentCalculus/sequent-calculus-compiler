use super::{Clause, Consumer, Covar, Ctor, Producer, Var};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt};

// Case
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Case {
    pub cases: Vec<Clause<Ctor>>,
}

impl std::fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pts_joined: String = self
            .cases
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "case {{ {} }}", pts_joined)
    }
}

impl FreeV for Case {
    fn free_vars(&self) -> HashSet<Var> {
        self.cases.free_vars()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.cases.free_covars()
    }
}

impl From<Case> for Consumer {
    fn from(value: Case) -> Self {
        Consumer::Case(value)
    }
}

impl Subst for Case {
    type Target = Case;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        Case {
            cases: self.cases.subst_sim(prod_subst, cons_subst),
        }
    }
}

#[cfg(test)]
mod case_test {
    use crate::syntax::Case;

    #[test]
    fn display() {
        let ex = Case { cases: vec![] };
        assert_eq!(format!("{ex}"), "case {  }".to_string());
    }
}
