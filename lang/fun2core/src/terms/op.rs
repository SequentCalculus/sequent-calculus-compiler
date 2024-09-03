use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::terms::Op {
    /// ```text
    /// 〚t_1 * t_2 〛_{c} = *( 〚t_1〛, 〚t_2〛; c)
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        core::syntax::Op {
            fst: Rc::new(self.fst.compile_opt(state)),
            op: self.op.compile(state),
            snd: Rc::new(self.snd.compile_opt(state)),
            continuation: Rc::new(cont),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::definition::CompileWithCont;
    use std::rc::Rc;

    fn example_op1() -> fun::syntax::terms::Op {
        fun::syntax::terms::Op {
            fst: Rc::new(fun::syntax::terms::Term::Lit(2)),
            op: fun::syntax::BinOp::Sub,
            snd: Rc::new(fun::syntax::terms::Term::Lit(1)),
        }
    }

    fn example_op2() -> fun::syntax::terms::Op {
        fun::syntax::terms::Op {
            fst: Rc::new(fun::syntax::terms::Term::Var("x".to_owned())),
            op: fun::syntax::BinOp::Prod,
            snd: Rc::new(
                fun::syntax::terms::Op {
                    fst: Rc::new(fun::syntax::terms::Term::Var("x".to_owned())),
                    op: fun::syntax::BinOp::Sub,
                    snd: Rc::new(fun::syntax::terms::Term::Lit(1)),
                }
                .into(),
            ),
        }
    }

    #[test]
    fn compile_op1() {
        let result = example_op1().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Op {
                    fst: Rc::new(core::syntax::Literal { lit: 2 }.into()),
                    op: core::syntax::BinOp::Sub,
                    snd: Rc::new(core::syntax::Literal { lit: 1 }.into()),
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
        .into();
        assert_eq!(result, expected);
    }

    #[test]
    fn compile_op2() {
        let result = example_op2().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
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
                        core::syntax::Mu {
                            covariable: "a1".to_owned(),
                            statement: Rc::new(
                                core::syntax::Op {
                                    fst: Rc::new(
                                        core::syntax::Variable {
                                            var: "x".to_owned(),
                                        }
                                        .into(),
                                    ),
                                    op: core::syntax::BinOp::Sub,
                                    snd: Rc::new(core::syntax::Literal { lit: 1 }.into()),
                                    continuation: Rc::new(
                                        core::syntax::Covariable {
                                            covar: "a1".to_owned(),
                                        }
                                        .into(),
                                    ),
                                }
                                .into(),
                            ),
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
        .into();
        assert_eq!(result, expected);
    }
}
