use crate::{
    naming_transformation::{bind_many, Bind, Continuation, NamingTransformation, TransformState},
    syntax::{Constructor, Covariable, Cut, Mu, MuTilde, Producer, Statement, Variable},
};
use std::rc::Rc;

impl NamingTransformation for Constructor {
    type Target = Producer;

    ///N(K(p_i; c_j)) = μa.bind(p_i)[λas.bind(c_j)[λbs.⟨K(as; bs) | a⟩]]
    fn transform(self, state: &mut TransformState) -> Producer {
        let new_covar = state.fresh_covar();
        let new_covar_clone = new_covar.clone();
        let new_statement = bind_many(
            self.producers.into(),
            Box::new(|vars, state: &mut TransformState| {
                bind_many(
                    self.consumers.into(),
                    Box::new(|covars, _: &mut TransformState| {
                        Cut {
                            producer: Rc::new(
                                Constructor {
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
                            consumer: Rc::new(Covariable { covar: new_covar }.into()),
                        }
                        .into()
                    }),
                    state,
                )
            }),
            state,
        );
        Mu {
            covariable: new_covar_clone,
            statement: Rc::new(new_statement),
        }
        .into()
    }
}

impl Bind for Constructor {
    ///bind(K(p_i; c_j))[k] = bind(p_i)[λas.bind(c_j)[λbs.⟨K(as; bs) | ~μx.k(x)⟩]]
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        let new_var = state.fresh_var();
        bind_many(
            self.producers.into(),
            Box::new(|vars, state: &mut TransformState| {
                bind_many(
                    self.consumers.into(),
                    Box::new(|covars, state: &mut TransformState| {
                        Cut {
                            producer: Rc::new(
                                Constructor {
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
                            consumer: Rc::new(
                                MuTilde {
                                    variable: new_var.clone(),
                                    statement: Rc::new(k(new_var, state)),
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
        syntax::{Constructor, Covariable, Ctor, Cut, Literal, Mu, MuTilde, Statement, Variable},
    };
    use std::rc::Rc;

    fn example_ctor1() -> Constructor {
        Constructor {
            id: Ctor::Nil,
            producers: vec![],
            consumers: vec![],
        }
    }

    fn example_ctor2() -> Constructor {
        Constructor {
            id: Ctor::Tup,
            producers: vec![
                Literal { lit: 1 }.into(),
                Variable {
                    var: "x".to_owned(),
                }
                .into(),
            ],
            consumers: vec![Covariable {
                covar: "a".to_owned(),
            }
            .into()],
        }
        //        Constructor {}
    }

    #[test]
    fn transform_ctor1() {
        let result = example_ctor1().transform(&mut Default::default());
        let expected = Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(
                        Constructor {
                            id: Ctor::Nil,
                            producers: vec![],
                            consumers: vec![],
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        Covariable {
                            covar: "a0".to_owned(),
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
    fn transform_ctor2() {
        let result = example_ctor2().transform(&mut Default::default());
        let expected = Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                Cut {
                    producer: Rc::new(Literal { lit: 1 }.into()),
                    consumer: Rc::new(
                        MuTilde {
                            variable: "x0".to_owned(),
                            statement: Rc::new(
                                Cut {
                                    producer: Rc::new(
                                        Constructor {
                                            id: Ctor::Tup,
                                            producers: vec![
                                                Variable {
                                                    var: "x0".to_owned(),
                                                }
                                                .into(),
                                                Variable {
                                                    var: "x".to_owned(),
                                                }
                                                .into(),
                                            ],
                                            consumers: vec![Covariable {
                                                covar: "a".to_owned(),
                                            }
                                            .into()],
                                        }
                                        .into(),
                                    ),
                                    consumer: Rc::new(
                                        Covariable {
                                            covar: "a0".to_owned(),
                                        }
                                        .into(),
                                    ),
                                }
                                .into(),
                            ),
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
    fn bind_ctor1() {
        let result =
            example_ctor1().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected = Cut {
            producer: Rc::new(
                Constructor {
                    id: Ctor::Nil,
                    producers: vec![],
                    consumers: vec![],
                }
                .into(),
            ),
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
    fn bind_cto2() {
        let result =
            example_ctor2().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected = Cut {
            producer: Rc::new(Literal { lit: 1 }.into()),
            consumer: Rc::new(
                MuTilde {
                    variable: "x1".to_owned(),
                    statement: Rc::new(
                        Cut {
                            producer: Rc::new(
                                Constructor {
                                    id: Ctor::Tup,
                                    producers: vec![
                                        Variable {
                                            var: "x1".to_owned(),
                                        }
                                        .into(),
                                        Variable {
                                            var: "x".to_owned(),
                                        }
                                        .into(),
                                    ],
                                    consumers: vec![Covariable {
                                        covar: "a".to_owned(),
                                    }
                                    .into()],
                                }
                                .into(),
                            ),
                            consumer: Rc::new(
                                MuTilde {
                                    variable: "x0".to_owned(),
                                    statement: Rc::new(Statement::Done()),
                                }
                                .into(),
                            ),
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
}
