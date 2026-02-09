//! This module defines the translation for the goto control operator.

use crate::{
    compile::{Compile, CompileState},
    names::compile_var,
    types::compile_ty,
};
use core_lang::syntax::terms::Cns;

impl Compile for fun::syntax::terms::Goto {
    /// This implementation of [Compile::compile_with_cont] proceeds as follows.
    /// ```text
    /// 〚goto a (t) 〛_{c} = 〚t〛_{a}
    /// ```
    ///
    /// # Panics
    ///
    /// A panic is caused if the types are not annotated in the program.
    fn compile_with_cont(
        self,
        _: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        self.term.compile_with_cont(
            core_lang::syntax::terms::XVar {
                prdcns: Cns,
                var: compile_var(self.target),
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
    use crate::compile::{Compile, CompileState};
    use fun::{parse_term, typing::check::Check};
    use macros::{covar, cut, ife, lit, mu, prod, ty, var};
    use std::collections::{HashSet, VecDeque};

    #[test]
    fn compile_goto_1() {
        let term = parse_term!("goto a (1)");
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_covar(
            fun::syntax::names::Var {
                name: "a".to_string(),
                id: 0,
            },
            fun::syntax::types::Ty::mk_i64(),
        );
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
        let result = term_typed.compile(&mut state, ty!("int"));
        let expected = mu!(("a", 0), cut!(lit!(1), covar!("a", 0))).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_goto_2() {
        let term = parse_term!("label a { if x == 0 {goto a (0)} else {x * 2} }");
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_var(
            fun::syntax::names::Var {
                name: "x".to_string(),
                id: 0,
            },
            fun::syntax::types::Ty::mk_i64(),
        );
        let term_typed = term
            .check(
                &mut Default::default(),
                &ctx,
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();

        let mut state = CompileState {
            used_vars: HashSet::from([
                core_lang::syntax::names::Var {
                    name: "x".to_string(),
                    id: 0,
                },
                core_lang::syntax::names::Var {
                    name: "a".to_string(),
                    id: 0,
                },
            ]),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term_typed.compile(&mut state, ty!("int"));

        let expected = mu!(
            ("a", 0),
            ife!(
                var!("x", 0),
                cut!(lit!(0), covar!("a", 0)),
                cut!(prod!(var!("x", 0), lit!(2)), covar!("a", 0))
            )
        )
        .into();
        assert_eq!(result, expected)
    }
}
