use super::{Consumer, Producer};
use std::fmt;

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

#[cfg(test)]
mod substitution_tests {
    use super::SubstitutionBinding;
    use crate::syntax::{Covariable, Variable};

    #[test]
    fn display_prod() {
        let result = format!(
            "{}",
            SubstitutionBinding::ProducerBinding(
                Variable {
                    var: "x".to_owned()
                }
                .into()
            )
        );
        let expected = "x";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_cons() {
        let result = format!(
            "{}",
            SubstitutionBinding::ConsumerBinding(
                Covariable {
                    covar: "a".to_owned()
                }
                .into()
            )
        );
        let expected = "'a";
        assert_eq!(result, expected)
    }
}
