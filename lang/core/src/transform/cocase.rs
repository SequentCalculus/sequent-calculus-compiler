use crate::{
    naming_transformation::{Bind, Continuation, NamingTransformation, TransformState},
    syntax::{Cocase, Cut, MuTilde, Statement},
};
use std::rc::Rc;

impl NamingTransformation for Cocase {
    type Target = Cocase;
    ///N(cocase {cocases}) = cocase { N(cocases) }
    fn transform(self, state: &mut TransformState) -> Cocase {
        Cocase {
            cocases: self.cocases.transform(state),
        }
    }
}

impl Bind for Cocase {
    ///bind(cocase {cocases)[k] = ⟨cocase N(cocases) | ~μx.k(x)⟩
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
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
        naming_transformation::{Bind, NamingTransformation, TransformState},
        syntax::{
            context::ContextBinding, types::Ty, Clause, Cocase, Covariable, Cut, Literal, MuTilde,
            Var, Variable,
        },
    };
    use std::rc::Rc;

    fn example_cocase1() -> Cocase {
        Cocase {
            cocases: vec![
                Clause {
                    xtor: "Hd".to_owned(),
                    context: vec![ContextBinding::CovarBinding {
                        covar: "a".to_owned(),
                        ty: Ty::Int(),
                    }],
                    rhs: Rc::new(
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
                },
                Clause {
                    xtor: "Tl".to_owned(),
                    context: vec![ContextBinding::CovarBinding {
                        covar: "a".to_owned(),
                        ty: Ty::Int(),
                    }],
                    rhs: Rc::new(
                        Cut {
                            producer: Rc::new(Literal { lit: 2 }.into()),
                            consumer: Rc::new(
                                Covariable {
                                    covar: "a".to_owned(),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                },
            ],
        }
    }
    fn example_cocase2() -> Cocase {
        Cocase {
            cocases: vec![
                Clause {
                    xtor: "Fst".to_owned(),
                    context: vec![ContextBinding::CovarBinding {
                        covar: "a".to_owned(),
                        ty: Ty::Int(),
                    }],
                    rhs: Rc::new(
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
                },
                Clause {
                    xtor: "Snd".to_owned(),
                    context: vec![ContextBinding::CovarBinding {
                        covar: "a".to_owned(),
                        ty: Ty::Int(),
                    }],
                    rhs: Rc::new(
                        Cut {
                            producer: Rc::new(Literal { lit: 2 }.into()),
                            consumer: Rc::new(
                                Covariable {
                                    covar: "a".to_owned(),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                },
            ],
        }
    }

    fn example_cocase3() -> Cocase {
        Cocase {
            cocases: vec![Clause {
                xtor: "Ap".to_owned(),
                context: vec![
                    ContextBinding::VarBinding {
                        var: "x".to_owned(),
                        ty: Ty::Int(),
                    },
                    ContextBinding::CovarBinding {
                        covar: "a".to_owned(),
                        ty: Ty::Int(),
                    },
                ],
                rhs: Rc::new(
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
            }],
        }
    }

    #[test]
    fn transform_cocase1() {
        let result = example_cocase1().transform(&mut Default::default());
        let expected = example_cocase1();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_cocase2() {
        let result = example_cocase2().transform(&mut Default::default());
        let expected = example_cocase2();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_cocase3() {
        let result = example_cocase3().transform(&mut Default::default());
        let expected = example_cocase3();
        assert_eq!(result, expected)
    }
    #[test]
    fn bind_cocase1() {
        let result = example_cocase1().bind(
            Box::new(|var: Var, _: &mut TransformState| {
                Cut {
                    producer: Rc::new(Variable { var }.into()),
                    consumer: Rc::new(
                        Covariable {
                            covar: "covar".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into()
            }),
            &mut Default::default(),
        );
        let expected = Cut {
            producer: Rc::new(example_cocase1().into()),
            consumer: Rc::new(
                MuTilde {
                    variable: "x0".into(),
                    statement: Rc::new(
                        Cut {
                            producer: Rc::new(
                                Variable {
                                    var: "x0".to_owned(),
                                }
                                .into(),
                            ),
                            consumer: Rc::new(
                                Covariable {
                                    covar: "covar".to_owned(),
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
    fn bind_cocase2() {
        let result = example_cocase2().bind(
            Box::new(|var: Var, _: &mut TransformState| {
                Cut {
                    producer: Rc::new(Variable { var }.into()),
                    consumer: Rc::new(
                        Covariable {
                            covar: "covar".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into()
            }),
            &mut Default::default(),
        );
        let expected = Cut {
            producer: Rc::new(example_cocase2().into()),
            consumer: Rc::new(
                MuTilde {
                    variable: "x0".into(),
                    statement: Rc::new(
                        Cut {
                            producer: Rc::new(
                                Variable {
                                    var: "x0".to_owned(),
                                }
                                .into(),
                            ),
                            consumer: Rc::new(
                                Covariable {
                                    covar: "covar".to_owned(),
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
    fn bind_cocase3() {
        let result = example_cocase3().bind(
            Box::new(|var: Var, _: &mut TransformState| {
                Cut {
                    producer: Rc::new(Variable { var }.into()),
                    consumer: Rc::new(
                        Covariable {
                            covar: "covar".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into()
            }),
            &mut Default::default(),
        );
        let expected = Cut {
            producer: Rc::new(example_cocase3().into()),
            consumer: Rc::new(
                MuTilde {
                    variable: "x0".into(),
                    statement: Rc::new(
                        Cut {
                            producer: Rc::new(
                                Variable {
                                    var: "x0".to_owned(),
                                }
                                .into(),
                            ),
                            consumer: Rc::new(
                                Covariable {
                                    covar: "covar".to_owned(),
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
