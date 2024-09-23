use crate::definition::{CompileState, CompileWithCont};
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::Let {
    /// ```text
    /// 〚let x := t_1 in t_2 〛_{c} = 〚t_1 〛_{μ~x.〚t_2 〛_{c}}
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        // new continuation: μ~x.〚t_2 〛_{c}
        let new_cont = core::syntax::MuTilde {
            variable: self.variable,
            statement: Rc::new(self.in_term.compile_with_cont(cont, state)),
        };

        // 〚t_1 〛_{new_cont}
        self.bound_term.compile_with_cont(new_cont.into(), state)
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::parse_term;

    use crate::definition::CompileWithCont;
    use std::rc::Rc;

    #[test]
    fn compile_let1() {
        let term = parse_term!("let x = 1 in x * x");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(core::syntax::Literal { lit: 1 }.into()),
                    consumer: Rc::new(
                        core::syntax::MuTilde {
                            variable: "x".to_owned(),
                            statement: Rc::new(
                                core::syntax::Op {
                                    fst: Rc::new(
                                        core::syntax::Variable {
                                            var: "x".to_owned(),
                                        }
                                        .into(),
                                    ),
                                    op: core::syntax::BinOp::Prod,
                                    snd: Rc::new(
                                        core::syntax::Variable {
                                            var: "x".to_owned(),
                                        }
                                        .into(),
                                    ),
                                    continuation: Rc::new(
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
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_let2() {
        let term = parse_term!("let x = Cons(x,Nil) in x");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(
                        core::syntax::Constructor {
                            id: core::syntax::Ctor::Cons,
                            producers: vec![
                                core::syntax::Variable {
                                    var: "x".to_owned(),
                                }
                                .into(),
                                core::syntax::Constructor {
                                    id: core::syntax::Ctor::Nil,
                                    producers: vec![],
                                    consumers: vec![],
                                }
                                .into(),
                            ],
                            consumers: vec![],
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        core::syntax::MuTilde {
                            variable: "x".to_owned(),
                            statement: Rc::new(
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
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
