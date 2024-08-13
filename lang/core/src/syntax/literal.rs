use super::{Consumer, Covar, Producer, Var};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt};
// Literal
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub lit: i64,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lit)
    }
}

impl FreeV for Literal {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        HashSet::new()
    }
}

impl From<Literal> for Producer {
    fn from(value: Literal) -> Self {
        Producer::Literal(value)
    }
}

impl Subst for Literal {
    type Target = Literal;

    fn subst_sim(
        &self,
        _prod_subst: &[(Producer, Var)],
        _cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        self.clone()
    }
}

#[cfg(test)]
mod literal_tests {
    use crate::syntax::Literal;

    #[test]
    fn display() {
        let ex = Literal { lit: 20 };
        assert_eq!(format!("{ex}"), "20".to_string())
    }
}
