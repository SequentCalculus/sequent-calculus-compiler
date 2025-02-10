use crate::definition::{Compile, CompileState, CompileWithCont};
use core_lang::syntax::{term::Cns, Ty};
use std::rc::Rc;

impl Compile for fun::syntax::terms::BinOp {
    type Target = core_lang::syntax::BinOp;
    fn compile(self, _state: &mut CompileState) -> Self::Target {
        match self {
            fun::syntax::terms::BinOp::Div => core_lang::syntax::BinOp::Div,
            fun::syntax::terms::BinOp::Prod => core_lang::syntax::BinOp::Prod,
            fun::syntax::terms::BinOp::Rem => core_lang::syntax::BinOp::Rem,
            fun::syntax::terms::BinOp::Sum => core_lang::syntax::BinOp::Sum,
            fun::syntax::terms::BinOp::Sub => core_lang::syntax::BinOp::Sub,
        }
    }
}

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
            fst: Rc::new(self.fst.compile_opt(state, Ty::I64)),
            op: self.op.compile(state),
            snd: Rc::new(self.snd.compile_opt(state, Ty::I64)),
            continuation: Rc::new(cont),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::{parse_term, typing::check::Check};

    use crate::definition::CompileWithCont;
    use core_lang::syntax::types::Ty;

    #[test]
    fn compile_op1() {
        let term = parse_term!("2 - 1");
        let result = term.compile_opt(&mut Default::default(), Ty::I64);
        let expected = core_lang::syntax::term::Mu::mu(
            "a0",
            core_lang::syntax::statement::Op::sub(
                core_lang::syntax::term::Literal::new(2),
                core_lang::syntax::term::Literal::new(1),
                core_lang::syntax::term::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
            ),
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected);
    }

    #[test]
    fn compile_op2() {
        let term = parse_term!("x * (x - 1)");
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_var("x", fun::syntax::types::Ty::mk_i64());
        let term_typed = term
            .check(
                &mut Default::default(),
                &ctx,
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();
        let result = term_typed.compile_opt(&mut Default::default(), Ty::I64);
        let expected = core_lang::syntax::term::Mu::mu(
            "a0",
            core_lang::syntax::statement::Op::prod(
                core_lang::syntax::term::XVar::var("x", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::term::Mu::mu(
                    "a1",
                    core_lang::syntax::statement::Op::sub(
                        core_lang::syntax::term::XVar::var("x", core_lang::syntax::types::Ty::I64),
                        core_lang::syntax::term::Literal::new(1),
                        core_lang::syntax::term::XVar::covar(
                            "a1",
                            core_lang::syntax::types::Ty::I64,
                        ),
                    ),
                    core_lang::syntax::types::Ty::I64,
                ),
                core_lang::syntax::term::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
            ),
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected);
    }
}
