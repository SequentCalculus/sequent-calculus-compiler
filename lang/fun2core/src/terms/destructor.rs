use crate::definition::{Compile, CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Destructor {
    /// ```text
    /// 〚t.D(t_1,...) 〛_{c} =  〚t〛_{D(〚t_1〛,...);c)}
    ///
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        // new continuation: D(〚t_1〛,...);c)
        let new_cont = core::syntax::Destructor {
            id: self.id.compile(st),
            producers: self.args.into_iter().map(|p| p.compile_opt(st)).collect(),
            consumers: vec![cont],
        }
        .into();
        // 〚t〛_{new_cont}
        self.destructee.compile_with_cont(new_cont, st)
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::definition::CompileWithCont;
    use std::rc::Rc;

    fn example_hd() -> fun::syntax::Destructor {
        fun::syntax::Destructor {
            id: fun::syntax::Dtor::Hd,
            destructee: Rc::new(
                fun::syntax::Cocase {
                    cocases: vec![
                        fun::syntax::Clause {
                            xtor: fun::syntax::Dtor::Hd,
                            vars: vec![],
                            rhs: fun::syntax::Term::Lit(1).into(),
                        },
                        fun::syntax::Clause {
                            xtor: fun::syntax::Dtor::Tl,
                            vars: vec![],
                            rhs: fun::syntax::Term::Lit(2).into(),
                        },
                    ],
                }
                .into(),
            ),
            args: vec![],
        }
    }

    fn example_tl() -> fun::syntax::Destructor {
        fun::syntax::Destructor {
            id: fun::syntax::Dtor::Tl,
            destructee: Rc::new(
                fun::syntax::Cocase {
                    cocases: vec![
                        fun::syntax::Clause {
                            xtor: fun::syntax::Dtor::Hd,
                            vars: vec![],
                            rhs: fun::syntax::Term::Lit(1).into(),
                        },
                        fun::syntax::Clause {
                            xtor: fun::syntax::Dtor::Tl,
                            vars: vec![],
                            rhs: fun::syntax::Term::Lit(2).into(),
                        },
                    ],
                }
                .into(),
            ),
            args: vec![],
        }
    }

    fn example_fst() -> fun::syntax::Destructor {
        fun::syntax::Destructor {
            id: fun::syntax::Dtor::Fst,
            args: vec![],
            destructee: Rc::new(
                fun::syntax::Cocase {
                    cocases: vec![
                        fun::syntax::Clause {
                            xtor: fun::syntax::Dtor::Fst,
                            vars: vec![],
                            rhs: fun::syntax::Term::Lit(1),
                        },
                        fun::syntax::Clause {
                            xtor: fun::syntax::Dtor::Snd,
                            vars: vec![],
                            rhs: fun::syntax::Term::Lit(2),
                        },
                    ],
                }
                .into(),
            ),
        }
    }

    fn example_snd() -> fun::syntax::Destructor {
        fun::syntax::Destructor {
            id: fun::syntax::Dtor::Snd,
            args: vec![],
            destructee: Rc::new(
                fun::syntax::Cocase {
                    cocases: vec![
                        fun::syntax::Clause {
                            xtor: fun::syntax::Dtor::Fst,
                            vars: vec![],
                            rhs: fun::syntax::Term::Lit(1),
                        },
                        fun::syntax::Clause {
                            xtor: fun::syntax::Dtor::Snd,
                            vars: vec![],
                            rhs: fun::syntax::Term::Lit(2),
                        },
                    ],
                }
                .into(),
            ),
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
                                            consumer: Rc::new(core::syntax::Consumer::Covar(
                                                "a1".to_owned(),
                                            )),
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
                                            consumer: Rc::new(core::syntax::Consumer::Covar(
                                                "a2".to_owned(),
                                            )),
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
                            consumers: vec![core::syntax::Consumer::Covar("a0".to_owned())],
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
                                            consumer: Rc::new(core::syntax::Consumer::Covar(
                                                "a1".to_owned(),
                                            )),
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
                                            consumer: Rc::new(core::syntax::Consumer::Covar(
                                                "a2".to_owned(),
                                            )),
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
                            consumers: vec![core::syntax::Consumer::Covar("a0".to_owned())],
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
                                            consumer: Rc::new(core::syntax::Consumer::Covar(
                                                "a1".to_owned(),
                                            )),
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
                                            consumer: Rc::new(core::syntax::Consumer::Covar(
                                                "a2".to_owned(),
                                            )),
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
                            consumers: vec![core::syntax::Consumer::Covar("a0".to_owned())],
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
                                            consumer: Rc::new(core::syntax::Consumer::Covar(
                                                "a1".to_owned(),
                                            )),
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
                                            consumer: Rc::new(core::syntax::Consumer::Covar(
                                                "a2".to_owned(),
                                            )),
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
                            consumers: vec![core::syntax::Consumer::Covar("a0".to_owned())],
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
