use crate::compile::{Compile, CompileState, CompileWithCont};
use core_lang::syntax::{terms::Cns, Ty};

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
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        core_lang::syntax::statements::Op {
            fst: Rc::new(self.fst.compile_opt(state, Ty::I64)),
            op: self.op.compile(state),
            snd: Rc::new(self.snd.compile_opt(state, Ty::I64)),
            next: Rc::new(cont),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::{parse_term, typing::check::Check};

    use crate::compile::{CompileState, CompileWithCont};
    use core_lang::syntax::types::Ty;

    use std::collections::{HashSet, VecDeque};

    #[test]
    fn compile_op1() {
        let term = parse_term!("2 - 1");

        let mut state = CompileState {
            used_vars: HashSet::default(),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term.compile_opt(&mut state, Ty::I64);

        let expected = core_lang::syntax::terms::Mu::mu(
            "a0",
            core_lang::syntax::statements::Op::sub(
                core_lang::syntax::terms::Literal::new(2),
                core_lang::syntax::terms::Literal::new(1),
                core_lang::syntax::terms::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
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

        let mut state = CompileState {
            used_vars: HashSet::from(["x".to_string()]),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term_typed.compile_opt(&mut state, Ty::I64);

        let expected = core_lang::syntax::terms::Mu::mu(
            "a0",
            core_lang::syntax::statements::Op::prod(
                core_lang::syntax::terms::XVar::var("x", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::terms::Mu::mu(
                    "a1",
                    core_lang::syntax::statements::Op::sub(
                        core_lang::syntax::terms::XVar::var("x", core_lang::syntax::types::Ty::I64),
                        core_lang::syntax::terms::Literal::new(1),
                        core_lang::syntax::terms::XVar::covar(
                            "a1",
                            core_lang::syntax::types::Ty::I64,
                        ),
                    ),
                    core_lang::syntax::types::Ty::I64,
                ),
                core_lang::syntax::terms::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
            ),
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected);
    }
}
