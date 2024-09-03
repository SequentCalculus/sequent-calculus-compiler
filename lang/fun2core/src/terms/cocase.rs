use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::terms::Cocase {
    /// ```text
    /// 〚cocase { D_1(x_11, ...) => t_1, ...} 〛_{c} = ⟨cocase{ D_1(x_11, ...; a_1) => 〚t_1〛_{a_1}, ... } | c⟩
    /// 〚cocase { D_1(x_11, ...) => t_1, ...} 〛 = cocase{ D_1(x_11, ...; a_1) => 〚t_1〛_{a_1}, ... }
    /// ```
    fn compile_opt(self, state: &mut CompileState) -> core::syntax::Producer {
        core::syntax::Cocase {
            cocases: self
                .cocases
                .into_iter()
                .map(|clause| compile_clause(clause, state))
                .collect(),
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        core::syntax::Cut {
            producer: Rc::new(self.compile_opt(state)),
            consumer: Rc::new(cont),
        }
        .into()
    }
}

fn compile_clause(
    clause: fun::syntax::terms::Clause<fun::syntax::Dtor>,
    state: &mut CompileState,
) -> core::syntax::Clause<core::syntax::Dtor> {
    let new_cv = state.free_covar_from_state();
    core::syntax::Clause {
        xtor: clause.xtor.compile(state),
        vars: clause.vars,
        covars: vec![new_cv.clone()],
        rhs: Rc::new(
            clause
                .rhs
                .compile_with_cont(core::syntax::Covariable { covar: new_cv }.into(), state),
        ),
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::definition::CompileWithCont;
    use std::rc::Rc;

    fn stream_example() -> fun::syntax::terms::Cocase {
        fun::syntax::terms::Cocase {
            cocases: vec![
                fun::syntax::terms::Clause {
                    xtor: fun::syntax::Dtor::Hd,
                    vars: vec![],
                    rhs: fun::syntax::terms::Term::Lit(1),
                },
                fun::syntax::terms::Clause {
                    xtor: fun::syntax::Dtor::Tl,
                    vars: vec![],
                    rhs: fun::syntax::terms::Term::Lit(2),
                },
            ],
        }
    }

    fn lpair_example() -> fun::syntax::terms::Cocase {
        fun::syntax::terms::Cocase {
            cocases: vec![
                fun::syntax::terms::Clause {
                    xtor: fun::syntax::Dtor::Fst,
                    vars: vec![],
                    rhs: fun::syntax::terms::Term::Lit(1),
                },
                fun::syntax::terms::Clause {
                    xtor: fun::syntax::Dtor::Snd,
                    vars: vec![],
                    rhs: fun::syntax::terms::Term::Lit(2),
                },
            ],
        }
    }

    #[test]
    fn compile_stream() {
        let result = stream_example().compile_opt(&mut Default::default());
        let expected = core::syntax::Cocase {
            cocases: vec![
                core::syntax::Clause {
                    xtor: core::syntax::Dtor::Hd,
                    vars: vec![],
                    covars: vec!["a0".to_owned()],
                    rhs: Rc::new(
                        core::syntax::Cut {
                            producer: Rc::new(core::syntax::Literal { lit: 1 }.into()),
                            consumer: Rc::new(
                                core::syntax::Covariable {
                                    covar: "a0".to_owned(),
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
                    covars: vec!["a1".to_owned()],
                    rhs: Rc::new(
                        core::syntax::Cut {
                            producer: Rc::new(core::syntax::Literal { lit: 2 }.into()),
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
            ],
        }
        .into();
        assert_eq!(result, expected);
    }

    #[test]
    fn complie_lpair() {
        let result = lpair_example().compile_opt(&mut Default::default());
        let expected = core::syntax::Cocase {
            cocases: vec![
                core::syntax::Clause {
                    xtor: core::syntax::Dtor::Fst,
                    vars: vec![],
                    covars: vec!["a0".to_owned()],
                    rhs: Rc::new(
                        core::syntax::Cut {
                            producer: Rc::new(core::syntax::Literal { lit: 1 }.into()),
                            consumer: Rc::new(
                                core::syntax::Covariable {
                                    covar: "a0".to_owned(),
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
                    covars: vec!["a1".to_owned()],
                    rhs: Rc::new(
                        core::syntax::Cut {
                            producer: Rc::new(core::syntax::Literal { lit: 2 }.into()),
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
            ],
        }
        .into();
        assert_eq!(result, expected);
    }
}
