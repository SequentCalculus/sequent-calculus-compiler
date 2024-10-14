use crate::definition::{Compile, CompileState, CompileWithCont};
use core::syntax::term::Cns;
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::Op {
    /// ```text
    /// 〚t_1 * t_2 〛_{c} = *( 〚t_1〛, 〚t_2〛; c)
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        core::syntax::statement::Op {
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
    use fun::parse_term;

    use crate::definition::CompileWithCont;
    use core::syntax::term::{Cns, Prd};
    use std::rc::Rc;

    #[test]
    fn compile_op1() {
        let term = parse_term!("2 - 1");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::statement::Op {
                    fst: Rc::new(core::syntax::term::Literal { lit: 2 }.into()),
                    op: core::syntax::BinOp::Sub,
                    snd: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                    continuation: Rc::new(
                        core::syntax::term::XVar {
                            prdcns: Cns,
                            var: "a0".to_owned(),
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
        let term = parse_term!("x * (x - 1)");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::statement::Op {
                    fst: Rc::new(
                        core::syntax::term::XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    op: core::syntax::BinOp::Prod,
                    snd: Rc::new(
                        core::syntax::term::Mu {
                            prdcns: Prd,
                            variable: "a1".to_owned(),
                            statement: Rc::new(
                                core::syntax::statement::Op {
                                    fst: Rc::new(
                                        core::syntax::term::XVar {
                                            prdcns: Prd,
                                            var: "x".to_owned(),
                                        }
                                        .into(),
                                    ),
                                    op: core::syntax::BinOp::Sub,
                                    snd: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                                    continuation: Rc::new(
                                        core::syntax::term::XVar {
                                            prdcns: Cns,
                                            var: "a1".to_owned(),
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
                        core::syntax::term::XVar {
                            prdcns: core::syntax::term::Cns,
                            var: "a0".to_owned(),
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
