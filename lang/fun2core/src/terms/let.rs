//! This module defines the translation of let-bindings.

use crate::{
    compile::{Compile, CompileState},
    types::compile_ty,
};
use core_lang::syntax::terms::Cns;

use std::rc::Rc;

impl Compile for fun::syntax::terms::Let {
    /// This implementation of [Compile::compile_with_cont] proceeds as follows.
    /// ```text
    /// 〚let x := t_1; t_2 〛_{c} = <〚t_1 〛| μ~x.〚t_2 〛_{c}>
    /// ```
    /// OR if `t_1: codata {...}`
    /// ```text
    /// 〚let x := t_1; t_2 〛_{c} = 〚t_1 〛_{μ~x.〚t_2 〛_{c}}
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        let ty = compile_ty(&self.var_ty);
        // new continuation: μ~x.〚t_2 〛_{c}
        let new_cont = core_lang::syntax::terms::Mu {
            prdcns: Cns,
            variable: core_lang::syntax::names::Var {
                name: self.variable,
                id: 0,
            },
            ty: ty.clone(),
            statement: Rc::new(self.in_term.compile_with_cont(cont, state)),
        }
        .into();

        if ty.is_codata(state.codata_types) {
            // <〚t_1 〛| new_cont>
            core_lang::syntax::statements::Cut {
                producer: Rc::new(self.bound_term.compile(state, ty.clone())),
                ty,
                consumer: Rc::new(new_cont),
            }
            .into()
        } else {
            // 〚t_1 〛_{new_cont}
            self.bound_term.compile_with_cont(new_cont, state)
        }
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::compile::{Compile, CompileState};
    use core_lang::syntax::names::Var;
    use core_macros::{covar, ctor, cut, lit, mu, mutilde, prod, ty, var};
    use fun::{parse_term, test_common::symbol_table_list, typing::check::Check};
    use std::collections::{HashSet, VecDeque};

    #[test]
    fn compile_let1() {
        let term = parse_term!("let x : i64 = 1; x * x");
        let term_typed = term
            .check(
                &mut Default::default(),
                &fun::syntax::context::TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();

        let mut state = CompileState {
            used_vars: HashSet::from([Var {
                name: "x".to_string(),
                id: 0,
            }]),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term_typed.compile(&mut state, ty!("int"));

        let expected = mu!(
            ("a", 0),
            cut!(
                lit!(1),
                mutilde!(
                    ("x", 0),
                    cut!(prod!(var!("x", 0), var!("x", 0)), covar!("a", 0))
                )
            )
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_let2() {
        let term = parse_term!("let x : List[i64] = Cons(x,Nil); x");
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_var("x", fun::syntax::types::Ty::mk_i64());
        let term_typed = term
            .check(
                &mut symbol_table_list(),
                &ctx,
                &fun::syntax::types::Ty::mk_decl(
                    "List",
                    fun::syntax::types::TypeArgs::mk(vec![fun::syntax::types::Ty::mk_i64()]),
                ),
            )
            .unwrap();

        let mut state = CompileState {
            used_vars: HashSet::from([Var {
                name: "x".to_string(),
                id: 0,
            }]),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term_typed.compile(&mut state, ty!("List[i64]"));

        let expected = mu!(
            ("a", 0),
            cut!(
                ctor!(
                    "Cons",
                    [var!("x", 0), ctor!("Nil", [], ty!("List[i64]"))],
                    ty!("List[i64]")
                ),
                mutilde!(
                    ("x", 1),
                    cut!(
                        var!("x", 1, ty!("List[i64]")),
                        covar!("a", 0, ty!("List[i64]")),
                        ty!("List[i64]")
                    ),
                    ty!("List[i64]")
                ),
                ty!("List[i64]")
            ),
            ty!("List[i64]")
        )
        .into();

        assert_eq!(result, expected)
    }
}
