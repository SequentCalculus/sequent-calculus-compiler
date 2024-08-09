use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Goto {
    /// ```text
    /// 〚goto(t;a) 〛_{c} = 〚t〛_{a}
    /// ```
    fn compile_with_cont(
        self,
        _: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        self.term
            .compile_with_cont(core::syntax::Consumer::Covar(self.target), st)
    }
}

#[cfg(test)]
mod compile_tests {

    use crate::definition::CompileWithCont;
    use std::rc::Rc;

    fn example_goto1() -> fun::syntax::Goto {
        fun::syntax::Goto {
            term: fun::syntax::Term::Lit(1).into(),
            target: "a".to_owned(),
        }
    }
    //label a => ifz(x,goto(a;0),x*2)
    fn example_goto2() -> fun::syntax::Label {
        fun::syntax::Label {
            label: "a".to_owned(),
            term: Rc::new(
                fun::syntax::IfZ {
                    ifc: fun::syntax::Term::Var("x".to_owned()).into(),
                    thenc: Rc::new(
                        fun::syntax::Goto {
                            term: Rc::new(fun::syntax::Term::Lit(0).into()),
                            target: "a".to_owned(),
                        }
                        .into(),
                    ),
                    elsec: Rc::new(
                        fun::syntax::Op {
                            fst: Rc::new(fun::syntax::Term::Var("x".to_owned())),
                            op: fun::syntax::BinOp::Prod,
                            snd: Rc::new(fun::syntax::Term::Lit(2)),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into()
    }

    #[test]
    fn compile_goto1() {
        let result = example_goto1().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(core::syntax::Literal { lit: 1 }.into()),
                    consumer: Rc::new(core::syntax::Consumer::Covar("a".to_owned())),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_goto2() {
        let result = example_goto2().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(
                        core::syntax::Mu {
                            covariable: "a".to_owned(),
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
                                            producer: Rc::new(
                                                core::syntax::Literal { lit: 0 }.into(),
                                            ),
                                            consumer: Rc::new(core::syntax::Consumer::Covar(
                                                "a".to_owned(),
                                            )),
                                        }
                                        .into(),
                                    ),
                                    elsec: Rc::new(
                                        core::syntax::Op {
                                            fst: Rc::new(
                                                core::syntax::Variable {
                                                    var: "x".to_owned(),
                                                }
                                                .into(),
                                            ),
                                            op: core::syntax::BinOp::Prod,
                                            snd: Rc::new(core::syntax::Literal { lit: 2 }.into()),
                                            continuation: Rc::new(core::syntax::Consumer::Covar(
                                                "a".to_owned(),
                                            )),
                                        }
                                        .into(),
                                    ),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(core::syntax::Consumer::Covar("a0".to_owned())),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
