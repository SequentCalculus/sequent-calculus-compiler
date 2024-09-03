use crate::definition::{CompileState, CompileWithCont};
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::IfZ {
    /// ```text
    /// 〚IfZ(t_1, t_2, t_3) 〛_{c} = IfZ(〚t_1 〛, 〚t_2 〛_{c}, 〚t_3 〛_{c})
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        core::syntax::IfZ {
            ifc: Rc::new(self.ifc.compile_opt(state)),
            thenc: Rc::new(self.thenc.compile_with_cont(cont.clone(), state)),
            elsec: Rc::new(self.elsec.compile_with_cont(cont, state)),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {

    use std::rc::Rc;

    use crate::definition::CompileWithCont;

    fn example_ifz1() -> fun::syntax::terms::IfZ {
        fun::syntax::terms::IfZ {
            ifc: Rc::new(fun::syntax::terms::Term::Lit(0)),
            thenc: Rc::new(fun::syntax::terms::Term::Lit(1)),
            elsec: Rc::new(fun::syntax::terms::Term::Lit(2)),
        }
    }

    fn example_ifz2() -> fun::syntax::terms::IfZ {
        fun::syntax::terms::IfZ {
            ifc: Rc::new(fun::syntax::terms::Term::Var("x".to_owned())),
            thenc: Rc::new(fun::syntax::terms::Term::Lit(1)),
            elsec: Rc::new(fun::syntax::terms::Term::Var("x".to_owned())),
        }
    }

    #[test]
    fn compile_ifz1() {
        let result = example_ifz1().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::IfZ {
                    ifc: Rc::new(core::syntax::Literal { lit: 0 }.into()),
                    thenc: Rc::new(
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
                    elsec: Rc::new(
                        core::syntax::Cut {
                            producer: Rc::new(core::syntax::Literal { lit: 2 }.into()),
                            consumer: Rc::new(
                                core::syntax::Covariable {
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
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_ifz2() {
        let result = example_ifz2().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::IfZ {
                    ifc: Rc::new(
                        core::syntax::Variable {
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    thenc: Rc::new(
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
                    elsec: Rc::new(
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
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
