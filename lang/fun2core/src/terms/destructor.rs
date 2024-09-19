use crate::definition::{Compile, CompileState, CompileWithCont};
use core::syntax::Covariable;
use fun::syntax::substitution::split_subst;

impl CompileWithCont for fun::syntax::terms::Destructor {
    /// ```text
    /// 〚t.D(t_1, ...) 〛_{c} = 〚t〛_{D(〚t_1〛, ...); c)}
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        let (pargs, cargs) = split_subst(self.args);
        state.covars.extend(cargs.clone());
        let mut consumers: Vec<core::syntax::Consumer> = cargs
            .into_iter()
            .map(|cv| Covariable { covar: cv }.into())
            .collect();
        consumers.push(cont);
        // new continuation: D(〚t_1〛, ...); c)
        let new_cont = core::syntax::Destructor {
            id: self.id.compile(state),
            producers: pargs.into_iter().map(|p| p.compile_opt(state)).collect(),
            consumers,
        }
        .into();

        // 〚t〛_{new_cont}
        self.destructee.compile_with_cont(new_cont, state)
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::definition::CompileWithCont;
    use std::rc::Rc;

    fn example_hd() -> fun::syntax::terms::Destructor {
        fun::syntax::terms::Destructor {
            id: fun::syntax::Dtor::Hd,
            destructee: Rc::new(
                fun::syntax::terms::Cocase {
                    cocases: vec![
                        fun::syntax::terms::Clause {
                            xtor: fun::syntax::Dtor::Hd,
                            context: vec![],
                            rhs: fun::syntax::terms::Term::Lit(1).into(),
                        },
                        fun::syntax::terms::Clause {
                            xtor: fun::syntax::Dtor::Tl,
                            context: vec![],
                            rhs: fun::syntax::terms::Term::Lit(2).into(),
                        },
                    ],
                }
                .into(),
            ),
            args: vec![],
        }
    }

    fn example_tl() -> fun::syntax::terms::Destructor {
        fun::syntax::terms::Destructor {
            id: fun::syntax::Dtor::Tl,
            destructee: Rc::new(
                fun::syntax::terms::Cocase {
                    cocases: vec![
                        fun::syntax::terms::Clause {
                            xtor: fun::syntax::Dtor::Hd,
                            context: vec![],
                            rhs: fun::syntax::terms::Term::Lit(1).into(),
                        },
                        fun::syntax::terms::Clause {
                            xtor: fun::syntax::Dtor::Tl,
                            context: vec![],
                            rhs: fun::syntax::terms::Term::Lit(2).into(),
                        },
                    ],
                }
                .into(),
            ),
            args: vec![],
        }
    }

    fn example_fst() -> fun::syntax::terms::Destructor {
        fun::syntax::terms::Destructor {
            id: fun::syntax::Dtor::Fst,
            args: vec![],
            destructee: Rc::new(
                fun::syntax::terms::Cocase {
                    cocases: vec![
                        fun::syntax::terms::Clause {
                            xtor: fun::syntax::Dtor::Fst,
                            context: vec![],
                            rhs: fun::syntax::terms::Term::Lit(1),
                        },
                        fun::syntax::terms::Clause {
                            xtor: fun::syntax::Dtor::Snd,
                            context: vec![],
                            rhs: fun::syntax::terms::Term::Lit(2),
                        },
                    ],
                }
                .into(),
            ),
        }
    }

    fn example_snd() -> fun::syntax::terms::Destructor {
        fun::syntax::terms::Destructor {
            id: fun::syntax::Dtor::Snd,
            args: vec![],
            destructee: Rc::new(
                fun::syntax::terms::Cocase {
                    cocases: vec![
                        fun::syntax::terms::Clause {
                            xtor: fun::syntax::Dtor::Fst,
                            context: vec![],
                            rhs: fun::syntax::terms::Term::Lit(1),
                        },
                        fun::syntax::terms::Clause {
                            xtor: fun::syntax::Dtor::Snd,
                            context: vec![],
                            rhs: fun::syntax::terms::Term::Lit(2),
                        },
                    ],
                }
                .into(),
            ),
        }
    }

    fn example_arg() -> fun::syntax::terms::Destructor {
        fun::syntax::terms::Destructor {
            id: fun::syntax::Dtor::Fst,
            args: vec![fun::syntax::terms::Term::Var("x".to_owned()).into()],
            destructee: Rc::new(fun::syntax::terms::Term::Var("x".to_owned())),
        }
    }

    #[test]
    fn compile_hd() {
        let result = example_hd().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(
                        core::syntax::Cocase {
                            cocases: vec![
                                core::syntax::Clause {
                                    xtor: core::syntax::Dtor::Hd,
                                    vars: vec![],
                                    covars: vec!["a1".to_owned()],
                                    rhs: Rc::new(
                                        core::syntax::Cut {
                                            producer: Rc::new(
                                                core::syntax::Literal { lit: 1 }.into(),
                                            ),
                                            consumer: Rc::new(
                                                core::syntax::Covariable {
                                                    covar: "a1".to_owned(),
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                },
                                core::syntax::Clause {
                                    xtor: core::syntax::Dtor::Tl,
                                    vars: vec![],
                                    covars: vec!["a2".to_owned()],
                                    rhs: Rc::new(
                                        core::syntax::Cut {
                                            producer: Rc::new(
                                                core::syntax::Literal { lit: 2 }.into(),
                                            ),
                                            consumer: Rc::new(
                                                core::syntax::Covariable {
                                                    covar: "a2".to_owned(),
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
                    consumer: Rc::new(
                        core::syntax::Destructor {
                            id: core::syntax::Dtor::Hd,
                            producers: vec![],
                            consumers: vec![core::syntax::Covariable {
                                covar: "a0".to_owned(),
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
    fn compile_tl() {
        let result = example_tl().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(
                        core::syntax::Cocase {
                            cocases: vec![
                                core::syntax::Clause {
                                    xtor: core::syntax::Dtor::Hd,
                                    vars: vec![],
                                    covars: vec!["a1".to_owned()],
                                    rhs: Rc::new(
                                        core::syntax::Cut {
                                            producer: Rc::new(
                                                core::syntax::Literal { lit: 1 }.into(),
                                            ),
                                            consumer: Rc::new(
                                                core::syntax::Covariable {
                                                    covar: "a1".to_owned(),
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                },
                                core::syntax::Clause {
                                    xtor: core::syntax::Dtor::Tl,
                                    vars: vec![],
                                    covars: vec!["a2".to_owned()],
                                    rhs: Rc::new(
                                        core::syntax::Cut {
                                            producer: Rc::new(
                                                core::syntax::Literal { lit: 2 }.into(),
                                            ),
                                            consumer: Rc::new(
                                                core::syntax::Covariable {
                                                    covar: "a2".to_owned(),
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
                    consumer: Rc::new(
                        core::syntax::Destructor {
                            id: core::syntax::Dtor::Tl,
                            producers: vec![],
                            consumers: vec![core::syntax::Covariable {
                                covar: "a0".to_owned(),
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
    fn compile_fst() {
        let result = example_fst().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(
                        core::syntax::Cocase {
                            cocases: vec![
                                core::syntax::Clause {
                                    xtor: core::syntax::Dtor::Fst,
                                    vars: vec![],
                                    covars: vec!["a1".to_owned()],
                                    rhs: Rc::new(
                                        core::syntax::Cut {
                                            producer: Rc::new(
                                                core::syntax::Literal { lit: 1 }.into(),
                                            ),
                                            consumer: Rc::new(
                                                core::syntax::Covariable {
                                                    covar: "a1".to_owned(),
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                },
                                core::syntax::Clause {
                                    xtor: core::syntax::Dtor::Snd,
                                    vars: vec![],
                                    covars: vec!["a2".to_owned()],
                                    rhs: Rc::new(
                                        core::syntax::Cut {
                                            producer: Rc::new(
                                                core::syntax::Literal { lit: 2 }.into(),
                                            ),
                                            consumer: Rc::new(
                                                core::syntax::Covariable {
                                                    covar: "a2".to_owned(),
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
                    consumer: Rc::new(
                        core::syntax::Destructor {
                            id: core::syntax::Dtor::Fst,
                            producers: vec![],
                            consumers: vec![core::syntax::Covariable {
                                covar: "a0".to_owned(),
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
    fn compile_snd() {
        let result = example_snd().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(
                        core::syntax::Cocase {
                            cocases: vec![
                                core::syntax::Clause {
                                    xtor: core::syntax::Dtor::Fst,
                                    vars: vec![],
                                    covars: vec!["a1".to_owned()],
                                    rhs: Rc::new(
                                        core::syntax::Cut {
                                            producer: Rc::new(
                                                core::syntax::Literal { lit: 1 }.into(),
                                            ),
                                            consumer: Rc::new(
                                                core::syntax::Covariable {
                                                    covar: "a1".to_owned(),
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                },
                                core::syntax::Clause {
                                    xtor: core::syntax::Dtor::Snd,
                                    vars: vec![],
                                    covars: vec!["a2".to_owned()],
                                    rhs: Rc::new(
                                        core::syntax::Cut {
                                            producer: Rc::new(
                                                core::syntax::Literal { lit: 2 }.into(),
                                            ),
                                            consumer: Rc::new(
                                                core::syntax::Covariable {
                                                    covar: "a2".to_owned(),
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
                    consumer: Rc::new(
                        core::syntax::Destructor {
                            id: core::syntax::Dtor::Snd,
                            producers: vec![],
                            consumers: vec![core::syntax::Covariable {
                                covar: "a0".to_owned(),
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
    fn compile_arg() {
        let result = example_arg().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(
                        core::syntax::Variable {
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        core::syntax::Destructor {
                            id: core::syntax::Dtor::Fst,
                            producers: vec![core::syntax::Variable {
                                var: "x".to_owned(),
                            }
                            .into()],
                            consumers: vec![core::syntax::Covariable {
                                covar: "a0".to_owned(),
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
}
