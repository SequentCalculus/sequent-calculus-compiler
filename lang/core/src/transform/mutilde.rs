use crate::syntax::statement::Cut;

use super::super::{
    naming_transformation::{Bind, Continuation, NamingTransformation, TransformState},
    syntax::{Mu, MuTilde, Statement},
};
use std::rc::Rc;

impl NamingTransformation for MuTilde {
    type Target = MuTilde;
    ///N(~μx.s) = ~μx.N(s)
    fn transform(self, state: &mut TransformState) -> MuTilde {
        state.used_vars.insert(self.variable.clone());
        MuTilde {
            variable: self.variable,
            statement: self.statement.transform(state),
        }
    }
}

impl Bind for MuTilde {
    /// bind(~μx.s)[k] = ⟨μa.k(a) | ~μx.N(s)⟩
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        state.used_vars.insert(self.variable.clone());
        let new_covar = state.fresh_covar();
        Cut {
            producer: Rc::new(
                Mu {
                    covariable: new_covar.clone(),
                    statement: Rc::new(k(new_covar, state)),
                }
                .into(),
            ),
            consumer: Rc::new(self.transform(state).into()),
        }
        .into()
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::{
        naming_transformation::{Bind, NamingTransformation},
        syntax::{statement::Cut, Covariable, Mu, MuTilde, Statement, Variable},
    };
    use std::rc::Rc;

    fn example_mutilde1() -> MuTilde {
        MuTilde {
            variable: "x".to_owned(),
            statement: Rc::new(Statement::Done()),
        }
    }
    fn example_mutilde2() -> MuTilde {
        MuTilde {
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
    }

    #[test]
    fn transform_mutilde1() {
        let result = example_mutilde1().transform(&mut Default::default());
        let expected = example_mutilde1();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_mutilde2() {
        let result = example_mutilde2().transform(&mut Default::default());
        let expected = example_mutilde2();
        assert_eq!(result, expected)
    }
    #[test]
    fn bind_mutilde1() {
        let result =
            example_mutilde1().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected = Cut {
            producer: Rc::new(
                Mu {
                    covariable: "a0".to_owned(),
                    statement: Rc::new(Statement::Done()),
                }
                .into(),
            ),
            consumer: Rc::new(example_mutilde1().into()),
        }
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn bind_mutilde2() {
        let result =
            example_mutilde2().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected = Cut {
            producer: Rc::new(
                Mu {
                    covariable: "a0".to_owned(),
                    statement: Rc::new(Statement::Done()),
                }
                .into(),
            ),
            consumer: Rc::new(example_mutilde2().into()),
        }
        .into();
        assert_eq!(result, expected)
    }
}
