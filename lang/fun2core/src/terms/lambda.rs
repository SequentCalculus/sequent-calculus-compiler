use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Lam {
    /// ```text
    /// 〚λx.t 〛_{c} = ⟨cocase { ap(x; a) => 〚t 〛_{a} } | c⟩
    /// 〚λx.t 〛 = cocase { ap(x; a) => 〚t 〛_{a} }
    /// ```
    fn compile_opt(self, state: &mut CompileState) -> core::syntax::Producer {
        let new_cv = state.free_covar_from_state();
        core::syntax::Cocase {
            cocases: vec![core::syntax::Clause {
                xtor: core::syntax::Dtor::Ap,
                vars: vec![self.variable],
                covars: vec![new_cv.clone()],
                rhs: Rc::new(
                    self.body.compile_with_cont(
                        core::syntax::Covariable { covar: new_cv }.into(),
                        state,
                    ),
                ),
            }],
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

#[cfg(test)]
mod compile_tests {
    use std::rc::Rc;

    use crate::definition::CompileWithCont;

    fn example_lam1() -> fun::syntax::Lam {
        fun::syntax::Lam {
            variable: "x".to_owned(),
            body: Rc::new(fun::syntax::Term::Var("x".to_owned())),
        }
    }

    fn example_lam2() -> fun::syntax::Lam {
        fun::syntax::Lam {
            variable: "x".to_owned(),
            body: Rc::new(
                fun::syntax::Lam {
                    variable: "y".to_owned(),
                    body: Rc::new(
                        fun::syntax::Op {
                            fst: Rc::new(fun::syntax::Term::Var("x".to_owned().into())),
                            op: fun::syntax::BinOp::Sum,
                            snd: Rc::new(fun::syntax::Term::Var("y".to_owned().into())),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
    }

    #[test]
    fn compile_lam1() {
        let result = example_lam1().compile_opt(&mut Default::default());
        let expected = core::syntax::Cocase {
            cocases: vec![core::syntax::Clause {
                xtor: core::syntax::Dtor::Ap,
                vars: vec!["x".to_owned()],
                covars: vec!["a0".to_owned()],
                rhs: Rc::new(
                    core::syntax::Cut {
                        producer: Rc::new(
                            core::syntax::Variable {
                                var: "x".to_owned(),
                            }
                            .into(),
                        ),
                        consumer: Rc::new(
                            core::syntax::Covariable {
                                covar: "a0".to_owned(),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                ),
            }],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn complie_lam2() {
        let result = example_lam2().compile_opt(&mut Default::default());
        let expected = core::syntax::Cocase {
            cocases: vec![core::syntax::Clause {
                xtor: core::syntax::Dtor::Ap,
                vars: vec!["x".to_owned()],
                covars: vec!["a0".to_owned()],
                rhs: Rc::new(
                    core::syntax::Cut {
                        producer: Rc::new(
                            core::syntax::Cocase {
                                cocases: vec![core::syntax::Clause {
                                    xtor: core::syntax::Dtor::Ap,
                                    vars: vec!["y".to_owned()],
                                    covars: vec!["a1".to_owned()],
                                    rhs: Rc::new(
                                        core::syntax::Op {
                                            fst: Rc::new(
                                                core::syntax::Variable {
                                                    var: "x".to_owned(),
                                                }
                                                .into(),
                                            ),
                                            op: core::syntax::BinOp::Sum,
                                            snd: Rc::new(
                                                core::syntax::Variable {
                                                    var: "y".to_owned(),
                                                }
                                                .into(),
                                            ),
                                            continuation: Rc::new(
                                                core::syntax::Covariable {
                                                    covar: "a1".to_owned(),
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                }],
                            }
                            .into(),
                        ),
                        consumer: Rc::new(
                            core::syntax::Covariable {
                                covar: "a0".to_owned(),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                ),
            }],
        }
        .into();
        assert_eq!(result, expected)
    }
}
