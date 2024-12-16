use crate::definition::{Compile, CompileState, CompileWithCont};
use core_lang::syntax::{term::Cns, types::Ty};
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::Op {
    /// ```text
    /// 〚t_1 * t_2 〛_{c} = *( 〚t_1〛, 〚t_2〛; c)
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        core_lang::syntax::statement::Op {
            fst: Rc::new(self.fst.compile_opt(state, Ty::Int)),
            op: self.op.compile(state),
            snd: Rc::new(self.snd.compile_opt(state, Ty::Int)),
            continuation: Rc::new(cont),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::{parse_term, typing::check::Check};

    use crate::definition::CompileWithCont;
    use core_lang::syntax::{term::Prd, types::Ty};
    use std::rc::Rc;

    #[test]
    fn compile_op1() {
        let term = parse_term!("2 - 1");
        let result = term.compile_opt(&mut Default::default(), Ty::Int);
        let expected = core_lang::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            ty: core_lang::syntax::types::Ty::Int,
            statement: Rc::new(
                core_lang::syntax::statement::Op {
                    fst: Rc::new(core_lang::syntax::term::Literal { lit: 2 }.into()),
                    op: core_lang::syntax::BinOp::Sub,
                    snd: Rc::new(core_lang::syntax::term::Literal { lit: 1 }.into()),
                    continuation: Rc::new(
                        core_lang::syntax::term::XVar::covar(
                            "a0",
                            core_lang::syntax::types::Ty::Int,
                        )
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
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_var("x", fun::syntax::types::Ty::mk_int());
        let term_typed = term
            .check(&Default::default(), &ctx, &fun::syntax::types::Ty::mk_int())
            .unwrap();
        let result = term_typed.compile_opt(&mut Default::default(), Ty::Int);
        let expected = core_lang::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            ty: core_lang::syntax::types::Ty::Int,
            statement: Rc::new(
                core_lang::syntax::statement::Op {
                    fst: Rc::new(
                        core_lang::syntax::term::XVar::var("x", core_lang::syntax::types::Ty::Int)
                            .into(),
                    ),
                    op: core_lang::syntax::BinOp::Prod,
                    snd: Rc::new(
                        core_lang::syntax::term::Mu {
                            prdcns: Prd,
                            variable: "a1".to_owned(),
                            ty: core_lang::syntax::types::Ty::Int,
                            statement: Rc::new(
                                core_lang::syntax::statement::Op {
                                    fst: Rc::new(
                                        core_lang::syntax::term::XVar::var(
                                            "x",
                                            core_lang::syntax::types::Ty::Int,
                                        )
                                        .into(),
                                    ),
                                    op: core_lang::syntax::BinOp::Sub,
                                    snd: Rc::new(
                                        core_lang::syntax::term::Literal { lit: 1 }.into(),
                                    ),
                                    continuation: Rc::new(
                                        core_lang::syntax::term::XVar::covar(
                                            "a1",
                                            core_lang::syntax::types::Ty::Int,
                                        )
                                        .into(),
                                    ),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                    continuation: Rc::new(
                        core_lang::syntax::term::XVar::covar(
                            "a0",
                            core_lang::syntax::types::Ty::Int,
                        )
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
