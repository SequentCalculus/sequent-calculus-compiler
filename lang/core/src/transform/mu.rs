use crate::syntax::statement::Cut;

use super::super::{
    naming_transformation::{Bind, Continuation, NamingTransformation, TransformState},
    syntax::{
        term::{Cns, Mu, Prd, PrdCns, Term},
        Statement,
    },
};
use std::rc::Rc;

impl<T: PrdCns> NamingTransformation for Mu<T> {
    type Target = Mu<T>;
    ///N(μa.s) = μa.N(s)
    fn transform(self, state: &mut TransformState) -> Self::Target {
        state.used_covars.insert(self.variable.clone());
        Mu {
            prdcns: self.prdcns,
            variable: self.variable,
            statement: self.statement.transform(state),
        }
    }
}

impl Bind for Mu<Prd> {
    ///bind(μa.s)[k] = ⟨μa.N(s) | ~μx.k(x)⟩
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        state.used_covars.insert(self.variable.clone());
        let new_var = state.fresh_var();
        Cut {
            producer: Rc::new(Term::Mu(self.transform(state)).into()),
            consumer: Rc::new(
                Mu {
                    prdcns: Cns,
                    variable: new_var.clone(),
                    statement: Rc::new(k(new_var, state)),
                }
                .into(),
            ),
        }
        .into()
    }
}

impl Bind for Mu<Cns> {
    /// bind(~μx.s)[k] = ⟨μa.k(a) | ~μx.N(s)⟩
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        state.used_vars.insert(self.variable.clone());
        let new_covar = state.fresh_covar();
        Cut {
            producer: Rc::new(
                Term::Mu(Mu {
                    prdcns: Prd,
                    variable: new_covar.clone(),
                    statement: Rc::new(k(new_covar, state)),
                })
                .into(),
            ),
            consumer: Rc::new(Term::Mu(self.transform(state)).into()),
        }
        .into()
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::{
        naming_transformation::{Bind, NamingTransformation},
        syntax::{
            statement::Cut,
            term::{Cns, Literal, Mu, Prd, XVar},
            Statement,
        },
    };
    use std::rc::Rc;

    fn example_mu1() -> Mu<Prd> {
        Mu {
            prdcns: Prd,
            variable: "a".to_owned(),
            statement: Rc::new(Statement::Done()),
        }
    }
    fn example_mu2() -> Mu<Prd> {
        Mu {
            prdcns: Prd,
            variable: "a".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(Literal { lit: 1 }.into()),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
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
                Mu {
                    prdcns: Cns,
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
                Mu {
                    prdcns: Cns,
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
