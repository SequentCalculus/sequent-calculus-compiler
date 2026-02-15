//! This module defines the translation of let-bindings.

use crate::{
    compile::{Compile, CompileState},
    types::compile_ty,
};
use core_lang::syntax::{names::Ident, terms::Cns};

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
            variable: Ident::new_with_zero(&self.variable),
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
    use core_macros::{covar, ctor, cut, id, lit, mu, mutilde, prod, ty, var};
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
            used_vars: HashSet::from(["x".to_string()]),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term_typed.compile(&mut state, ty!("int"));

        let expected = mu!(
            id!("a0"),
            cut!(
                lit!(1),
                mutilde!(
                    id!("x"),
                    cut!(prod!(var!(id!("x")), var!(id!("x"))), covar!(id!("a0")))
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
            used_vars: HashSet::from(["x".to_string()]),
            codata_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term_typed.compile(&mut state, ty!(id!("List[i64]")));

        let expected = mu!(
            id!("a0"),
            cut!(
                ctor!(
                    id!("Cons"),
                    [var!(id!("x")), ctor!(id!("Nil"), [], ty!(id!("List[i64]")))],
                    ty!(id!("List[i64]"))
                ),
                mutilde!(
                    id!("x"),
                    cut!(
                        var!(id!("x"), ty!(id!("List[i64]"))),
                        covar!(id!("a0"), ty!(id!("List[i64]"))),
                        ty!(id!("List[i64]"))
                    ),
                    ty!(id!("List[i64]"))
                ),
                ty!(id!("List[i64]"))
            ),
            ty!(id!("List[i64]"))
        )
        .into();

        assert_eq!(result, expected)
    }
}
