use super::{Consumer, Covar, Producer, Var};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt};

// Variable
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub var: Var,
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.var)
    }
}

impl FreeV for Variable {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::from([self.var.clone()])
    }

    fn free_covars(&self) -> HashSet<Covar> {
        HashSet::new()
    }
}

impl From<Variable> for Producer {
    fn from(value: Variable) -> Self {
        Producer::Variable(value)
    }
}

impl Subst for Variable {
    type Target = Producer;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        _cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        let Variable { var } = self;
        match prod_subst.iter().find(|(_, v)| v == var) {
            None => Variable { var: var.clone() }.into(),
            Some((p, _)) => p.clone(),
        }
    }
}

#[cfg(test)]
mod variable_tests {
    use std::collections::HashSet;

    use crate::{syntax::Variable, traits::free_vars::FreeV};

    #[test]
    fn display() {
        let ex = Variable {
            var: "x".to_string(),
        };
        assert_eq!(format!("{ex}"), "x")
    }

    #[test]
    fn free_vars() {
        let ex = Variable {
            var: "x".to_string(),
        };
        let mut res = HashSet::new();
        res.insert("x".to_string());
        assert_eq!(ex.free_vars(), res)
    }

    #[test]
    fn free_covars() {
        let ex = Variable {
            var: "x".to_string(),
        };
        assert_eq!(ex.free_covars(), HashSet::new())
    }
}
