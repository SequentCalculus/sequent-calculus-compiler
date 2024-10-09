use super::{PrdCns, Term};
use crate::{
    syntax::{Covar, Statement, Var},
    traits::free_vars::FreeV,
};
use std::{collections::HashSet, fmt, rc::Rc};

// Mu
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mu<T: PrdCns> {
    pub prdcns: T,
    pub variable: Var,
    pub statement: Rc<Statement>,
}

impl<T: PrdCns> std::fmt::Display for Mu<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = if self.prdcns.is_prd() {
            format!("mu '{}", self.variable)
        } else {
            format!("mutilde {}", self.variable)
        };
        write!(f, "{}. {}", prefix, self.statement)
    }
}

impl<T: PrdCns> FreeV for Mu<T> {
    fn free_vars(&self) -> HashSet<Var> {
        let mut free_vars = FreeV::free_vars(Rc::as_ref(&self.statement));
        if self.prdcns.is_cns() {
            free_vars.remove(&self.variable);
        }
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let mut free_covars = self.statement.free_covars();
        if self.prdcns.is_prd() {
            free_covars.remove(&self.variable);
        }
        free_covars
    }
}

impl<T: PrdCns> From<Mu<T>> for Term<T> {
    fn from(value: Mu<T>) -> Self {
        Term::Mu(value)
    }
}

#[cfg(test)]
mod mu_tests {
    use super::{FreeV, Mu};
    use crate::syntax::{
        statement::Cut,
        term::{Cns, Prd},
        Covariable, Variable,
    };
    use std::{collections::HashSet, rc::Rc};

    fn example_mu() -> Mu<Prd> {
        Mu {
            prdcns: Prd,
            variable: "a".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(
                        Variable {
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        Covariable {
                            covar: "a".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into()
    }

    fn example_mu_tilde() -> Mu<Cns> {
        Mu {
            prdcns: Cns,
            variable: "x".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(
                        Variable {
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        Covariable {
                            covar: "a".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into()
    }

    #[test]
    fn display_mu() {
        let result = format!("{}", example_mu());
        let expected = "mu 'a. <x | 'a>".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_mu_tilde() {
        let result = format!("{}", example_mu_tilde());
        let expected = "mutilde x. <x | 'a>".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_mu() {
        let result = example_mu().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_mu_tilde() {
        let result = example_mu_tilde().free_vars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_mu() {
        let result = example_mu().free_covars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_mu_tilde() {
        let result = example_mu_tilde().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }
}
