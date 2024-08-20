use super::super::{
    naming_transformation::{bind_many, Bind, Continuation, NamingTransformation, TransformState},
    syntax::{Consumer, Covariable, Cut, Destructor, Mu, MuTilde, Statement, Variable},
};
use std::rc::Rc;

impl NamingTransformation for Destructor {
    type Target = Consumer;
    ///N(D(p_i; cj)) =  ~μx.bind(p_i)[λas.bind(c_j)[λbs.⟨x | D(as; bs)⟩]]
    fn transform(self, state: &mut TransformState) -> Consumer {
        let new_var = state.fresh_var();
        let new_var_clone = new_var.clone();
        let new_statement = bind_many(
            self.producers.into(),
            Box::new(|vars, state: &mut TransformState| {
                bind_many(
                    self.consumers.into(),
                    Box::new(|covars, _: &mut TransformState| {
                        Cut {
                            producer: Rc::new(Variable { var: new_var }.into()),
                            consumer: Rc::new(
                                Destructor {
                                    id: self.id,
                                    producers: vars
                                        .into_iter()
                                        .map(|var| Variable { var }.into())
                                        .collect(),
                                    consumers: covars
                                        .into_iter()
                                        .map(|covar| Covariable { covar }.into())
                                        .collect(),
                                }
                                .into(),
                            ),
                        }
                        .into()
                    }),
                    state,
                )
            }),
            state,
        );
        MuTilde {
            variable: new_var_clone,
            statement: Rc::new(new_statement),
        }
        .into()
    }
}

impl Bind for Destructor {
    ///bind(D(p_i; c_j))[k] = bind(p_i)[λas.bind(c_j)[λbs.⟨μa.k(a) | D(as; bs)⟩]]
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        let new_covar = state.fresh_covar();
        bind_many(
            self.producers.into(),
            Box::new(|vars, state: &mut TransformState| {
                bind_many(
                    self.consumers.into(),
                    Box::new(|covars, state: &mut TransformState| {
                        Cut {
                            producer: Rc::new(
                                Mu {
                                    covariable: new_covar.clone(),
                                    statement: Rc::new(k(new_covar, state)),
                                }
                                .into(),
                            ),
                            consumer: Rc::new(
                                Destructor {
                                    id: self.id,
                                    producers: vars
                                        .into_iter()
                                        .map(|var| Variable { var }.into())
                                        .collect(),
                                    consumers: covars
                                        .into_iter()
                                        .map(|covar| Covariable { covar }.into())
                                        .collect(),
                                }
                                .into(),
                            ),
                        }
                        .into()
                    }),
                    state,
                )
            }),
            state,
        )
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::{
        naming_transformation::{Bind, NamingTransformation},
        syntax::{Covariable, Cut, Destructor, Dtor, Mu, MuTilde, Statement, Variable},
    };
    use std::rc::Rc;

    fn example_dtor1() -> Destructor {
        Destructor {
            id: Dtor::Hd,
            producers: vec![],
            consumers: vec![Covariable {
                covar: "a".to_owned(),
            }
            .into()],
        }
    }
    fn example_dtor2() -> Destructor {
        Destructor {
            id: Dtor::Ap,
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

    #[test]
    // μx  ̃ .bind(p i ) [λas.bind(c j ) [λbs.⟨x | D (as; bs)⟩]]
    fn transform_dtor1() {
        let result = example_dtor1().transform(&mut Default::default());
        let expected = MuTilde {
            variable: "x0".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(
                        Variable {
                            var: "x0".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        Destructor {
                            id: Dtor::Hd,
                            producers: vec![],
                            consumers: vec![Covariable {
                                covar: "a".to_owned(),
                            }
                            .into()],
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_dtor2() {
        let result = example_dtor2().transform(&mut Default::default());
        let expected = MuTilde {
            variable: "x0".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(
                        Variable {
                            var: "x0".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        Destructor {
                            id: Dtor::Ap,
                            producers: vec![Variable {
                                var: "x".to_owned(),
                            }
                            .into()],
                            consumers: vec![Covariable {
                                covar: "a".to_owned(),
                            }
                            .into()],
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_dtor1() {
        let result =
            example_dtor1().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected = Cut {
            producer: Rc::new(
                Mu {
                    covariable: "a0".to_owned(),
                    statement: Rc::new(Statement::Done()),
                }
                .into(),
            ),
            consumer: Rc::new(
                Destructor {
                    id: Dtor::Hd,
                    producers: vec![],
                    consumers: vec![Covariable {
                        covar: "a".to_owned(),
                    }
                    .into()],
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_dtor2() {
        let result =
            example_dtor2().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected = Cut {
            producer: Rc::new(
                Mu {
                    covariable: "a0".to_owned(),
                    statement: Rc::new(Statement::Done()),
                }
                .into(),
            ),
            consumer: Rc::new(
                Destructor {
                    id: Dtor::Ap,
                    producers: vec![Variable {
                        var: "x".to_owned(),
                    }
                    .into()],
                    consumers: vec![Covariable {
                        covar: "a".to_owned(),
                    }
                    .into()],
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
