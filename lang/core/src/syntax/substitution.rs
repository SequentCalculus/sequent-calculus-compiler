use super::{Consumer, Covar, Producer, Var};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SubstitutionBinding {
    ProducerBinding(Producer),
    ConsumerBinding(Consumer),
}

pub type Substitution = Vec<SubstitutionBinding>;

impl fmt::Display for SubstitutionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubstitutionBinding::ProducerBinding(prod) => prod.fmt(f),
            SubstitutionBinding::ConsumerBinding(cons) => cons.fmt(f),
        }
    }
}

impl From<Producer> for SubstitutionBinding {
    fn from(prod: Producer) -> SubstitutionBinding {
        SubstitutionBinding::ProducerBinding(prod)
    }
}

impl From<Consumer> for SubstitutionBinding {
    fn from(cons: Consumer) -> SubstitutionBinding {
        SubstitutionBinding::ConsumerBinding(cons)
    }
}

impl FreeV for SubstitutionBinding {
    fn free_vars(&self) -> HashSet<Var> {
        match self {
            SubstitutionBinding::ProducerBinding(prod) => prod.free_vars(),
            SubstitutionBinding::ConsumerBinding(cons) => cons.free_vars(),
        }
    }
    fn free_covars(&self) -> HashSet<Covar> {
        match self {
            SubstitutionBinding::ProducerBinding(prod) => prod.free_covars(),
            SubstitutionBinding::ConsumerBinding(cons) => cons.free_covars(),
        }
    }
}

impl Subst for SubstitutionBinding {
    type Target = SubstitutionBinding;
    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        match self {
            SubstitutionBinding::ProducerBinding(prod) => {
                SubstitutionBinding::ProducerBinding(prod.subst_sim(prod_subst, cons_subst))
            }
            SubstitutionBinding::ConsumerBinding(cons) => {
                SubstitutionBinding::ConsumerBinding(cons.subst_sim(prod_subst, cons_subst))
            }
        }
    }
}

#[cfg(test)]
mod substitution_tests {
    use super::{Substitution, SubstitutionBinding};
    use crate::{
        syntax::{Consumer, Covar, Covariable, Producer, Var, Variable},
        traits::{free_vars::FreeV, substitution::Subst},
    };
    use std::collections::HashSet;

    fn example_prod() -> SubstitutionBinding {
        SubstitutionBinding::ProducerBinding(
            Variable {
                var: "x".to_owned(),
            }
            .into(),
        )
    }

    fn example_cons() -> SubstitutionBinding {
        SubstitutionBinding::ConsumerBinding(
            Covariable {
                covar: "a".to_owned(),
            }
            .into(),
        )
    }

    fn example_subst() -> Substitution {
        vec![example_prod(), example_cons()]
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
    fn display_prod() {
        let result = format!("{}", example_prod());
        let expected = "x";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_cons() {
        let result = format!("{}", example_cons());
        let expected = "'a";
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars() {
        let result = example_subst().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars() {
        let result = example_subst().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn substitution() {
        let result = example_subst().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = vec![
            SubstitutionBinding::ProducerBinding(
                Variable {
                    var: "y".to_owned(),
                }
                .into(),
            ),
            SubstitutionBinding::ConsumerBinding(
                Covariable {
                    covar: "b".to_owned(),
                }
                .into(),
            ),
        ];
        assert_eq!(result, expected)
    }
}
