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
    use fun::parse_term;

    use crate::definition::CompileWithCont;
    use core::syntax::{context::ContextBinding, types::Ty};
    use std::rc::Rc;

    #[test]
    fn compile_fst() {
        let term = parse_term!("cocase { fst => 1, snd => 2}.fst");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(
                        core::syntax::Cocase {
                            cocases: vec![
                                core::syntax::Clause {
                                    xtor: core::syntax::Dtor::Fst,
                                    context: vec![ContextBinding::CovarBinding {
                                        covar: "a1".to_owned(),
                                        ty: Ty::Int(),
                                    }],
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
                                    context: vec![ContextBinding::CovarBinding {
                                        covar: "a2".to_owned(),
                                        ty: Ty::Int(),
                                    }],
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
        let term = parse_term!("cocase { fst => 1, snd => 2}.snd");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(
                        core::syntax::Cocase {
                            cocases: vec![
                                core::syntax::Clause {
                                    xtor: core::syntax::Dtor::Fst,
                                    context: vec![ContextBinding::CovarBinding {
                                        covar: "a1".to_owned(),
                                        ty: Ty::Int(),
                                    }],
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
                                    context: vec![ContextBinding::CovarBinding {
                                        covar: "a2".to_owned(),
                                        ty: Ty::Int(),
                                    }],
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
}
