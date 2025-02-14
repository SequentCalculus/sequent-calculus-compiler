use crate::{
    compile::{CompileState, CompileWithCont},
    program::compile_ty,
};
use core_lang::syntax::terms::Cns;

impl CompileWithCont for fun::syntax::terms::Goto {
    /// ```text
    /// 〚goto(t; a) 〛_{c} = 〚t〛_{a}
    /// ```
    fn compile_with_cont(
        self,
        _: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        self.term.compile_with_cont(
            core_lang::syntax::terms::XVar {
                prdcns: Cns,
                var: self.target,
                ty: compile_ty(
                    &self
                        .ty
                        .expect("Types should be annotated before translation"),
                ),
            }
            .into(),
            state,
        )
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::compile::CompileWithCont;
    use fun::{parse_term, typing::check::Check};

    #[test]
    fn compile_goto1() {
        let term = parse_term!("goto(1; a)");
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_covar("a", fun::syntax::types::Ty::mk_i64());
        let term_typed = term
            .check(
                &mut Default::default(),
                &ctx,
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::I64);
        let expected = core_lang::syntax::terms::Mu::mu(
            "a0",
            core_lang::syntax::statements::Cut::new(
                core_lang::syntax::terms::Literal::new(1),
                core_lang::syntax::terms::XVar::covar("a", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::types::Ty::I64,
            ),
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_goto2() {
        let term = parse_term!("label a { if x == 0 {goto(0;a)} else {x * 2} }");
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_var("x", fun::syntax::types::Ty::mk_i64());
        let term_typed = term
            .check(
                &mut Default::default(),
                &ctx,
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::I64);
        let expected = core_lang::syntax::terms::Mu::mu(
            "a",
            core_lang::syntax::statements::IfZ::new(
                core_lang::syntax::terms::XVar::var("x", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::statements::Cut::new(
                    core_lang::syntax::terms::Literal::new(0),
                    core_lang::syntax::terms::XVar::covar("a", core_lang::syntax::types::Ty::I64),
                    core_lang::syntax::types::Ty::I64,
                ),
                core_lang::syntax::statements::Op::prod(
                    core_lang::syntax::terms::XVar::var("x", core_lang::syntax::types::Ty::I64),
                    core_lang::syntax::terms::Literal::new(2),
                    core_lang::syntax::terms::XVar::covar("a", core_lang::syntax::types::Ty::I64),
                ),
            ),
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }
}
