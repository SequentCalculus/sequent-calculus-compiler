//! This module defines the translation of let-bindings.

use crate::{
    compile::{Compile, CompileState},
    types::compile_ty,
};
use core_lang::syntax::{
    names::Identifier,
    terms::Cns,
    type_params::{ParamPolarity, TypeParam},
};

use std::{collections::HashMap, rc::Rc};

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
        type_params: Rc<HashMap<String, (Identifier, ParamPolarity)>>,
    ) -> core_lang::syntax::Statement {
        let ty = compile_ty(&self.var_ty, type_params.clone());
        // new continuation: μ~x.〚t_2 〛_{c}
        let new_cont = core_lang::syntax::terms::Mu {
            prdcns: Cns,
            variable: Identifier::new(self.variable),
            ty: ty.clone(),
            statement: Rc::new(
                self.in_term
                    .compile_with_cont(cont, state, type_params.clone()),
            ),
        }
        .into();

        // Resolve `ty`'s polarity via the ambient declared type parameters currently in scope.
        let ambient_type_params: Vec<TypeParam> = type_params
            .values()
            .map(|(id, polarity)| TypeParam {
                id: id.clone(),
                polarity: *polarity,
            })
            .collect();

        if ty.is_codata(state.codata_types, &ambient_type_params) {
            // <〚t_1 〛| new_cont>
            core_lang::syntax::statements::Cut {
                producer: Rc::new(self.bound_term.compile(state, ty.clone(), type_params)),
                ty,
                consumer: Rc::new(new_cont),
            }
            .into()
        } else {
            // 〚t_1 〛_{new_cont}
            self.bound_term
                .compile_with_cont(new_cont, state, type_params)
        }
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::compile::{Compile, CompileState};
    use core_lang::syntax::type_params::ParamPolarity;
    use core_macros::{
        bind, covar, ctor, ctor_sig, cut, data, id, lit, mu, mutilde, prd, prod, ty, var,
    };
    use fun::{
        parse_term, syntax::util::dummy_span, test_common::symbol_table_list, typing::check::Check,
    };
    use std::{
        collections::{HashMap, HashSet, VecDeque},
        rc::Rc,
    };

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
            data_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
            max_id: &mut 0,
        };
        let result = term_typed.compile(&mut state, ty!("int"), Rc::default());

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

        let list = data!(
            id!("List[i64]"),
            [
                ctor_sig!(id!("Nil", 1), [], []),
                ctor_sig!(
                    id!("Cons", 2),
                    [],
                    [
                        bind!(id!("x"), prd!()),
                        bind!(id!("xs"), prd!(), ty!(id!("List"), vec![ty!("int")])),
                    ]
                )
            ],
            []
        );

        let mut state = CompileState {
            used_vars: HashSet::from(["x".to_string()]),
            codata_types: &[],
            data_types: &[list],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
            max_id: &mut 0,
        };
        let result = term_typed.compile(
            &mut state,
            ty!(id!("List"), vec![ty!("int")]),
            Rc::default(),
        );

        let expected = mu!(
            id!("a0"),
            cut!(
                ctor!(
                    id!("Cons", 2),
                    [],
                    [
                        var!(id!("x")),
                        ctor!(id!("Nil", 1), [], [], ty!(id!("List"), vec![ty!("int")]))
                    ],
                    ty!(id!("List"), vec![ty!("int")])
                ),
                mutilde!(
                    id!("x"),
                    cut!(
                        var!(id!("x"), ty!(id!("List"), vec![ty!("int")])),
                        covar!(id!("a0"), ty!(id!("List"), vec![ty!("int")])),
                        ty!(id!("List"), vec![ty!("int")])
                    ),
                    ty!(id!("List"), vec![ty!("int")])
                ),
                ty!(id!("List"), vec![ty!("int")])
            ),
            ty!(id!("List"), vec![ty!("int")])
        )
        .into();

        assert_eq!(result, expected)
    }

    /// Builds `let x: A = f(); x` where `A` is a bare type parameter, with `f`'s Call term used as
    /// the bound term specifically because `Call::compile` does *not* override the default
    /// Mu-wrapping `Compile::compile`, so the CBV and CBN paths produce structurally different
    /// results: CBV wraps the compiled call in an extra `Cut`/`Mu`, CBN compiles it directly
    /// against the continuation with no wrapper.
    fn let_with_call_bound_to_type_param() -> fun::syntax::terms::Let {
        fun::syntax::terms::Let {
            span: dummy_span(),
            variable: "x".to_string(),
            var_ty: fun::syntax::types::Ty::mk_decl("A", fun::syntax::types::TypeArgs::default()),
            bound_term: Rc::new(
                fun::syntax::terms::Call {
                    span: dummy_span(),
                    name: "f".to_string(),
                    type_args: fun::syntax::types::TypeArgs::default(),
                    args: fun::syntax::arguments::Arguments::default(),
                    ret_ty: Some(fun::syntax::types::Ty::mk_decl(
                        "A",
                        fun::syntax::types::TypeArgs::default(),
                    )),
                }
                .into(),
            ),
            in_term: Rc::new(
                fun::syntax::terms::XVar {
                    span: dummy_span(),
                    var: "x".to_string(),
                    ty: Some(fun::syntax::types::Ty::mk_decl(
                        "A",
                        fun::syntax::types::TypeArgs::default(),
                    )),
                    chi: Some(fun::syntax::context::Chirality::Prd),
                }
                .into(),
            ),
            ty: None,
        }
    }

    #[test]
    fn compile_let_uses_cut_branch_for_codata_declared_type_param() {
        // This is the direct regression test for the polarity-annotation bug: before the fix,
        // `is_codata` always defaulted an unresolved `Ty::Var` to `false`/positive here,
        // silently forcing call-by-value even when the type parameter was declared `-`/codata.
        let term = let_with_call_bound_to_type_param();

        let fresh_a = id!("A", 1);
        let mut subst = HashMap::new();
        subst.insert("A".to_string(), (fresh_a.clone(), ParamPolarity::Codata));

        let mut state = CompileState {
            used_vars: HashSet::from(["x".to_string()]),
            codata_types: &[],
            data_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
            max_id: &mut 0,
        };

        let cont: core_lang::syntax::terms::Term<core_lang::syntax::Cns> =
            covar!(id!("a0"), core_lang::syntax::types::Ty::Var(fresh_a)).into();
        let result = term.compile_with_cont(cont, &mut state, Rc::new(subst));

        // A codata-declared type parameter makes `ty.is_codata(...)` resolve to `true`, so the
        // `let` takes the `Cut`-with-`Mu`-producer branch. Before the fix, `is_codata` always
        // defaulted an unresolved `Ty::Var` to `false`/data here, silently taking the other
        // branch regardless of the declared polarity.
        match result {
            core_lang::syntax::Statement::Cut(cut) => {
                assert!(
                    matches!(cut.producer.as_ref(), core_lang::syntax::terms::Term::Mu(_)),
                    "expected the Cut's producer to be a Mu-wrapped call"
                );
            }
            other => panic!(
                "expected a Cut statement for a codata-declared type parameter, got {other:?}"
            ),
        }
    }

    #[test]
    fn compile_let_uses_direct_branch_for_data_declared_type_param() {
        let term = let_with_call_bound_to_type_param();

        let fresh_a = id!("A", 1);
        let mut subst = HashMap::new();
        subst.insert("A".to_string(), (fresh_a.clone(), ParamPolarity::Data));

        let mut state = CompileState {
            used_vars: HashSet::from(["x".to_string()]),
            codata_types: &[],
            data_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
            max_id: &mut 0,
        };

        let cont: core_lang::syntax::terms::Term<core_lang::syntax::Cns> =
            covar!(id!("a0"), core_lang::syntax::types::Ty::Var(fresh_a)).into();
        let result = term.compile_with_cont(cont, &mut state, Rc::new(subst));

        // A data-declared type parameter makes `ty.is_codata(...)` resolve to `false`, so the
        // `let` takes the direct `compile_with_cont` branch, i.e. the result is the `Call`
        // statement itself, not wrapped in a `Cut`.
        assert!(
            matches!(result, core_lang::syntax::Statement::Call(_)),
            "expected a direct Call statement for a data-declared type parameter, got {result:?}"
        );
    }
}
