use crate::syntax::statement::Cut;

use super::super::{
    naming_transformation::{Bind, Continuation, NamingTransformation, TransformState},
    syntax::{Mu, MuTilde, Statement},
};
use std::rc::Rc;

impl NamingTransformation for Mu {
    type Target = Mu;
    ///N(μa.s) = μa.N(s)
    fn transform(self, state: &mut TransformState) -> Mu {
        state.used_covars.insert(self.covariable.clone());
        Mu {
            covariable: self.covariable,
            statement: self.statement.transform(state),
        }
    }
}

impl Bind for Mu {
    ///bind(μa.s)[k] = ⟨μa.N(s) | ~μx.k(x)⟩
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        state.used_covars.insert(self.covariable.clone());
        let new_var = state.fresh_var();
        Cut {
            producer: Rc::new(self.transform(state).into()),
            consumer: Rc::new(
                MuTilde {
                    variable: new_var.clone(),
                    statement: Rc::new(k(new_var, state)),
                }
                .into(),
            ),
        }
        .into()
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::{
        naming_transformation::{Bind, NamingTransformation},
        syntax::{statement::Cut, Covariable, Literal, Mu, MuTilde, Statement},
    };
    use std::rc::Rc;

    fn example_mu1() -> Mu {
        Mu {
            covariable: "a".to_owned(),
            statement: Rc::new(Statement::Done()),
        }
    }
    fn example_mu2() -> Mu {
        Mu {
            covariable: "a".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(Literal { lit: 1 }.into()),
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
    }

    #[test]
    fn transform_mu1() {
        let result = example_mu1().transform(&mut Default::default());
        let expected = example_mu1();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_mu2() {
        let result = example_mu2().transform(&mut Default::default());
        let expected = example_mu2();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_mu1() {
        let result =
            example_mu1().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected = Cut {
            producer: Rc::new(example_mu1().into()),
            consumer: Rc::new(
                MuTilde {
                    variable: "x0".to_owned(),
                    statement: Rc::new(Statement::Done()),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn bind_mu2() {
        let result =
            example_mu2().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected = Cut {
            producer: Rc::new(example_mu2().into()),
            consumer: Rc::new(
                MuTilde {
                    variable: "x0".to_owned(),
                    statement: Rc::new(Statement::Done()),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
