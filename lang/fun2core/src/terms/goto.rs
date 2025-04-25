use crate::{
    compile::{CompileState, CompileWithCont},
    types::compile_ty,
};
use core_lang::syntax::terms::Cns;

impl CompileWithCont for fun::syntax::terms::Goto {
    /// ```text
    /// 〚goto a (t) 〛_{c} = 〚t〛_{a}
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
    use crate::compile::{CompileState, CompileWithCont};
    use fun::{parse_term, typing::check::Check};

    use std::collections::{HashSet, VecDeque};

    #[test]
    fn compile_goto_1() {
        let term = parse_term!("goto a (1)");
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_covar("a", fun::syntax::types::Ty::mk_i64());
        let term_typed = term
            .check(
                &mut Default::default(),
                &ctx,
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();
        let mut state = CompileState {
            used_vars: HashSet::default(),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term_typed.compile_opt(&mut state, core_lang::syntax::types::Ty::I64);
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
    fn compile_goto_2() {
        let term = parse_term!("label a { if x == 0 {goto a (0)} else {x * 2} }");
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_var("x", fun::syntax::types::Ty::mk_i64());
        let term_typed = term
            .check(
                &mut Default::default(),
                &ctx,
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();

        let mut state = CompileState {
            used_vars: HashSet::from(["x".to_string(), "a".to_string()]),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term_typed.compile_opt(&mut state, core_lang::syntax::types::Ty::I64);

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
