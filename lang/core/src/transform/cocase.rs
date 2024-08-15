use crate::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Cocase, Cut, MuTilde, Name, Statement},
};
use std::rc::Rc;

impl NamingTransformation for Cocase {
    type Target = Cocase;
    ///N (cocase {cocases}) = cocase { N(cocases) }
    fn transform(self, st: &mut TransformState) -> Cocase {
        Cocase {
            cocases: self.cocases.transform(st),
        }
    }
}

impl Bind for Cocase {
    ///bind(cocase {cocases) [k] = ⟨cocase N(cocases) | μxk (x)⟩
    fn bind<F, K>(self, k: F, st: &mut TransformState) -> Statement
    where
        F: FnOnce(Name) -> K,
        K: FnOnce(&mut TransformState) -> Statement,
    {
        let new_v = st.fresh_var();
        Cut {
            producer: Rc::new(self.transform(st).into()),
            consumer: Rc::new(
                MuTilde {
                    variable: new_v.clone(),
                    statement: Rc::new(k(new_v)(st)),
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
        syntax::{Clause, Cocase, Covariable, Cut, Dtor, Literal, MuTilde, Var, Variable},
    };
    use std::rc::Rc;

    fn example_cocase1() -> Cocase {
        Cocase {
            cocases: vec![
                Clause {
                    xtor: Dtor::Hd,
                    vars: vec![],
                    covars: vec!["a".to_owned()],
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
                    xtor: Dtor::Tl,
                    vars: vec![],
                    covars: vec!["a".to_owned()],
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
                    xtor: Dtor::Fst,
                    vars: vec![],
                    covars: vec!["a".to_owned()],
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
                    xtor: Dtor::Snd,
                    vars: vec![],
                    covars: vec!["a".to_owned()],
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
                xtor: Dtor::Ap,
                vars: vec!["x".to_owned()],
                covars: vec!["a".to_owned()],
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
            |a: Var| {
                |_: &mut TransformState| {
                    Cut {
                        producer: Rc::new(Variable { var: a }.into()),
                        consumer: Rc::new(
                            Covariable {
                                covar: "covar".to_owned(),
                            }
                            .into(),
                        ),
                    }
                    .into()
                }
            },
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
            |a: Var| {
                |_: &mut TransformState| {
                    Cut {
                        producer: Rc::new(Variable { var: a }.into()),
                        consumer: Rc::new(
                            Covariable {
                                covar: "covar".to_owned(),
                            }
                            .into(),
                        ),
                    }
                    .into()
                }
            },
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
            |a: Var| {
                |_: &mut TransformState| {
                    Cut {
                        producer: Rc::new(Variable { var: a }.into()),
                        consumer: Rc::new(
                            Covariable {
                                covar: "covar".to_owned(),
                            }
                            .into(),
                        ),
                    }
                    .into()
                }
            },
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
