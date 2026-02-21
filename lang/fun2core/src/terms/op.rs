//! This module defines the translation of arithmetic binary operations.

use crate::compile::{Compile, CompileState};
use core_lang::syntax::{
    Ty,
    terms::{Cns, Prd},
};

use std::rc::Rc;

/// This function converts [arithmetic binary operations in Fun](fun::syntax::terms::BinOp) to
/// [arithmetic binary operations in Core](core_lang::syntax::BinOp).
fn compile_op(op: fun::syntax::terms::BinOp) -> core_lang::syntax::BinOp {
    match op {
        fun::syntax::terms::BinOp::Div => core_lang::syntax::BinOp::Div,
        fun::syntax::terms::BinOp::Prod => core_lang::syntax::BinOp::Prod,
        fun::syntax::terms::BinOp::Rem => core_lang::syntax::BinOp::Rem,
        fun::syntax::terms::BinOp::Sum => core_lang::syntax::BinOp::Sum,
        fun::syntax::terms::BinOp::Sub => core_lang::syntax::BinOp::Sub,
    }
}

impl Compile for fun::syntax::terms::Op {
    /// This implementation of [Compile::compile] proceeds as follows.
    /// ```text
    /// 〚t_1 * t_2 〛= *( 〚t_1〛, 〚t_2〛)
    /// ```
    fn compile(
        self,
        state: &mut crate::compile::CompileState,
        _ty: Ty,
    ) -> core_lang::syntax::terms::Term<Prd> {
        core_lang::syntax::terms::Op {
            fst: Rc::new(self.fst.compile(state, Ty::I64)),
            op: compile_op(self.op),
            snd: Rc::new(self.snd.compile(state, Ty::I64)),
        }
        .into()
    }

    /// This implementation of [Compile::compile_with_cont] proceeds as follows.
    /// ```text
    /// 〚t_1 * t_2 〛_{c} = ⟨*( 〚t_1〛, 〚t_2〛) | c⟩
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        let new_op: core_lang::syntax::terms::Term<Prd> = core_lang::syntax::terms::Op {
            fst: Rc::new(self.fst.compile(state, Ty::I64)),
            op: compile_op(self.op),
            snd: Rc::new(self.snd.compile(state, Ty::I64)),
        }
        .into();
        core_lang::syntax::statements::Cut {
            producer: Rc::new(new_op),
            ty: Ty::I64,
            consumer: Rc::new(cont),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::compile::{Compile, CompileState};
    use core_macros::{id, lit, prod, sub, ty, var};
    use fun::{parse_term, typing::check::Check};

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
        let result = term.compile(&mut state, ty!("int"));

        let expected = sub!(lit!(2), lit!(1),).into();
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
        let result = term_typed.compile(&mut state, ty!("int"));

        let expected = prod!(var!(id!("x")), sub!(var!(id!("x")), lit!(1))).into();
        assert_eq!(result, expected);
    }
}
