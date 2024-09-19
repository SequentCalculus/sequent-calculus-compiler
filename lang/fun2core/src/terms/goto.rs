use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::terms::Goto {
    /// ```text
    /// 〚goto(t; a) 〛_{c} = 〚t〛_{a}
    /// ```
    fn compile_with_cont(
        self,
        _: core::syntax::Consumer,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        self.term.compile_with_cont(
            core::syntax::Covariable { covar: self.target }.into(),
            state,
        )
    }
}

#[cfg(test)]
mod compile_tests {

    use fun::parse_term;

    use crate::definition::CompileWithCont;
    use std::rc::Rc;

    #[test]
    fn compile_goto1() {
        let term = parse_term!("goto(1; 'a)");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(core::syntax::Literal { lit: 1 }.into()),
                    consumer: Rc::new(
                        core::syntax::Covariable {
                            covar: "a".to_owned(),
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
    fn compile_goto2() {
        let term = parse_term!("label 'a { ifz(x, goto(0;'a), x * 2) }");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
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
                            producer: Rc::new(core::syntax::Literal { lit: 0 }.into()),
                            consumer: Rc::new(
                                core::syntax::Covariable {
                                    covar: "a".to_owned(),
                                }
                                .into(),
                            ),
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
                            continuation: Rc::new(
                                core::syntax::Covariable {
                                    covar: "a".to_owned(),
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
