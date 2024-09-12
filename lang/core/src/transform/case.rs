use crate::{
    naming_transformation::{Bind, Continuation, NamingTransformation, TransformState},
    syntax::{Case, Cut, Mu, Statement},
};
use std::rc::Rc;

impl NamingTransformation for Case {
    type Target = Case;

    ///N(case {cases}) = case { N(cases) }
    fn transform(self, state: &mut TransformState) -> Case {
        Case {
            cases: self.cases.transform(state),
        }
    }
}

impl Bind for Case {
    ///bind(case {cases)[k] =  ⟨μa.k(a) | case N{cases}⟩
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        let new_covar = state.fresh_covar();
        Cut {
            consumer: Rc::new(
                Case {
                    cases: self.cases.transform(state),
                }
                .into(),
            ),
            producer: Rc::new(
                Mu {
                    covariable: new_covar.clone(),
                    statement: Rc::new(k(new_covar, state)),
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
        syntax::{
            context::ContextBinding, types::Ty, Case, Clause, Covariable, Ctor, Cut, Literal, Mu,
            Var, Variable,
        },
    };
    use std::rc::Rc;

    fn example_case1() -> Case {
        Case {
            cases: vec![
                Clause {
                    xtor: Ctor::Nil,
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
                    xtor: Ctor::Cons,
                    context: vec![
                        ContextBinding::VarBinding {
                            var: "x".to_owned(),
                            ty: Ty::Int(),
                        },
                        ContextBinding::VarBinding {
                            var: "xs".to_owned(),
                            ty: Ty::Decl("Listint".to_owned()),
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
                },
            ],
        }
    }
    fn example_case2() -> Case {
        Case {
            cases: vec![Clause {
                xtor: Ctor::Tup,
                context: vec![
                    ContextBinding::VarBinding {
                        var: "x".to_owned(),
                        ty: Ty::Int(),
                    },
                    ContextBinding::VarBinding {
                        var: "y".to_owned(),
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
    fn transform_case1() {
        let result = example_case1().transform(&mut Default::default());
        let expected = example_case1();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_case1() {
        let result = example_case1().bind(
            Box::new(|var: Var, _| {
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
            producer: Rc::new(
                Mu {
                    covariable: "a0".to_owned(),
                    statement: Rc::new(
                        Cut {
                            producer: Rc::new(
                                Variable {
                                    var: "a0".to_owned(),
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
            consumer: Rc::new(
                Case {
                    cases: vec![
                        Clause {
                            xtor: Ctor::Nil,
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
                            xtor: Ctor::Cons,
                            context: vec![
                                ContextBinding::VarBinding {
                                    var: "x".to_owned(),
                                    ty: Ty::Int(),
                                },
                                ContextBinding::VarBinding {
                                    var: "xs".to_owned(),
                                    ty: Ty::Decl("Listint".to_owned()),
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
                        },
                    ],
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected);
    }

    #[test]
    fn transform_case2() {
        let result = example_case2().transform(&mut Default::default());
        let expected_transform = example_case2();
        assert_eq!(result, expected_transform)
    }

    #[test]
    fn bind_case2() {
        let result = example_case2().bind(
            Box::new(|a, _| {
                Cut {
                    producer: Rc::new(Variable { var: a }.into()),
                    consumer: Rc::new(
                        Covariable {
                            covar: "covar".into(),
                        }
                        .into(),
                    ),
                }
                .into()
            }),
            &mut Default::default(),
        );
        let expected = Cut {
            producer: Rc::new(
                Mu {
                    covariable: "a0".to_owned(),
                    statement: Rc::new(
                        Cut {
                            producer: Rc::new(
                                Variable {
                                    var: "a0".to_owned(),
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
            consumer: Rc::new(
                Case {
                    cases: vec![Clause {
                        xtor: Ctor::Tup,
                        context: vec![
                            ContextBinding::VarBinding {
                                var: "x".to_owned(),
                                ty: Ty::Int(),
                            },
                            ContextBinding::VarBinding {
                                var: "y".to_owned(),
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
