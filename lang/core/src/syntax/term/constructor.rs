use super::{Cns, Prd, PrdCns};
use crate::{
    syntax::{stringify_and_join, substitution::Substitution, Name},
    traits::{free_vars::FreeV, substitution::Subst},
};
use std::{collections::HashSet, fmt};

// Constructor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xtor<T: PrdCns> {
    pub prdcns: T,
    pub id: Name,
    pub args: Substitution,
}

impl<T: PrdCns> std::fmt::Display for Xtor<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_joined: String = stringify_and_join(&self.args);
        write!(f, "{}({})", self.id, args_joined)
    }
}

impl FreeV for Constructor {
    fn free_vars(&self) -> HashSet<Var> {
        self.args.free_vars()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.args.free_covars()
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
            args: self.args.subst_sim(prod_subst, cons_subst),
        }
    }
}

#[cfg(test)]
mod constructor_tests {
    use crate::{
        syntax::{
            substitution::SubstitutionBinding, Constructor, Consumer, Covar, Covariable, Producer,
            Var, Variable,
        },
        traits::{free_vars::FreeV, substitution::Subst},
    };
    use std::collections::HashSet;

    fn example_cons() -> Constructor {
        Constructor {
            id: "Cons".to_owned(),
            args: vec![
                Into::<Producer>::into(Variable {
                    var: "x".to_owned(),
                })
                .into(),
                Into::<Consumer>::into(Covariable {
                    covar: "a".to_owned(),
                })
                .into(),
            ],
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
        let expected = "Cons(x, 'a)".to_owned();
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
            id: "Cons".to_owned(),
            args: vec![
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
            ],
        };
        assert_eq!(result, expected)
    }
}
