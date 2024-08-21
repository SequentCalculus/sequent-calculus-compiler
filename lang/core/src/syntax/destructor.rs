use super::{stringify_and_join, Consumer, Covar, Dtor, Producer, Var};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt};

// Destructor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Destructor {
    pub id: Dtor,
    pub producers: Vec<Producer>,
    pub consumers: Vec<Consumer>,
}

impl std::fmt::Display for Destructor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_joined: String = stringify_and_join(&self.producers);
        let coargs_joined: String = stringify_and_join(&self.consumers);
        write!(f, "{}({}; {})", self.id, args_joined, coargs_joined)
    }
}

impl FreeV for Destructor {
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

impl From<Destructor> for Consumer {
    fn from(value: Destructor) -> Self {
        Consumer::Destructor(value)
    }
}

impl Subst for Destructor {
    type Target = Destructor;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        Destructor {
            id: self.id.clone(),
            producers: self.producers.subst_sim(prod_subst, cons_subst),
            consumers: self.consumers.subst_sim(prod_subst, cons_subst),
        }
    }
}

#[cfg(test)]
mod destructor_tests {
    use crate::{
        syntax::{Consumer, Covar, Covariable, Destructor, Dtor, Producer, Var, Variable},
        traits::{free_vars::FreeV, substitution::Subst},
    };
    use std::collections::HashSet;

    fn example_dest() -> Destructor {
        Destructor {
            id: Dtor::Hd,
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
    fn display_dest() {
        let result = format!("{}", example_dest());
        let expected = "hd(x; a)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_dest() {
        let result = example_dest().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_dest() {
        let result = example_dest().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_dest() {
        let result = example_dest().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Destructor {
            id: Dtor::Hd,
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
