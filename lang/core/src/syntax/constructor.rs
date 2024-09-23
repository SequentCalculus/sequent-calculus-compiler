use super::{stringify_and_join, Consumer, Covar, Ctor, Producer, Var};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt};

// Constructor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor {
    pub id: Ctor,
    pub producers: Vec<Producer>,
    pub consumers: Vec<Consumer>,
}

impl std::fmt::Display for Constructor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_joined: String = stringify_and_join(&self.producers);
        let coargs_joined: String = stringify_and_join(&self.consumers);
        write!(f, "{}({}; {})", self.id, args_joined, coargs_joined)
    }
}

impl FreeV for Constructor {
    fn free_vars(&self) -> HashSet<Var> {
        let mut free_vars = self.producers.free_vars();
        free_vars.extend(self.consumers.free_vars());
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let mut free_covars = self.producers.free_covars();
        free_covars.extend(self.consumers.free_covars());
        free_covars
    }
}

impl From<Constructor> for Producer {
    fn from(value: Constructor) -> Self {
        Producer::Constructor(value)
    }
}

impl Subst for Constructor {
    type Target = Constructor;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        Constructor {
            id: self.id.clone(),
            producers: self.producers.subst_sim(prod_subst, cons_subst),
            consumers: self.consumers.subst_sim(prod_subst, cons_subst),
        }
    }
}

#[cfg(test)]
mod constructor_tests {
    use crate::{
        syntax::{Constructor, Consumer, Covar, Covariable, Ctor, Producer, Var, Variable},
        traits::{free_vars::FreeV, substitution::Subst},
    };
    use std::collections::HashSet;

    fn example_cons() -> Constructor {
        Constructor {
            id: Ctor::Cons,
            producers: vec![Variable {
                var: "x".to_owned(),
            }
            .into()],
            consumers: vec![Covariable {
                covar: "a".to_owned(),
            }
            .into()],
        }
    }

    fn example_prodsubst() -> Vec<(Producer, Var)> {
        vec![(
            Variable {
                var: "y".to_owned(),
            }
            .into(),
            "x".to_owned(),
        )]
    }
    fn example_conssubst() -> Vec<(Consumer, Covar)> {
        vec![(
            Covariable {
                covar: "b".to_owned(),
            }
            .into(),
            "a".to_owned(),
        )]
    }

    #[test]
    fn display_cons() {
        let result = format!("{}", example_cons());
        let expected = "Cons(x; 'a)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_cons() {
        let result = example_cons().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_cons() {
        let result = example_cons().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_cons() {
        let result = example_cons().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Constructor {
            id: Ctor::Cons,
            producers: vec![Variable {
                var: "y".to_owned(),
            }
            .into()],
            consumers: vec![Covariable {
                covar: "b".to_owned(),
            }
            .into()],
        };
        assert_eq!(result, expected)
    }
}
