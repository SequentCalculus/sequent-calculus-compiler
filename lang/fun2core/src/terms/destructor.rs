//! This module defines the translation of a destructor.

use crate::{
    arguments::compile_subst,
    compile::{Compile, CompileState},
    types::compile_ty,
};
use core_lang::syntax::terms::Cns;
use fun::syntax::types::OptTyped;

impl Compile for fun::syntax::terms::Destructor {
    /// This implementation of [Compile::compile_with_cont] proceeds as follows.
    /// ```text
    /// 〚t.D(t_1, ...) 〛_{c} = 〚t〛_{D(〚t_1〛, ..., c)}
    /// ```
    ///
    /// # Panics
    ///
    /// A panic is caused if the types are not annotated in the program.
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        let mut args = compile_subst(self.args, state);
        args.add_cons(cont);
        // new continuation: D(〚t_1〛, ..., c)
        let new_cont = core_lang::syntax::terms::Xtor {
            prdcns: Cns,
            id: self.id,
            args,
            ty: compile_ty(
                &self
                    .scrutinee
                    .get_type()
                    .expect("Types should be annotated before translation"),
            ),
        }
        .into();

        // 〚t〛_{new_cont}
        self.scrutinee.compile_with_cont(new_cont, state)
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::{parse_term, test_common::symbol_table_lpair, typing::check::Check};

    use crate::compile::{Compile, CompileState};
    use core_lang::syntax::{
        terms::{Cns, Prd},
        types::Ty,
    };

    use std::collections::{HashSet, VecDeque};
    use std::rc::Rc;

    #[test]
    fn compile_fst() {
        let term = parse_term!("new { fst => 1, snd => 2}.fst[i64, i64]");
        let term_typed = term
            .check(
                &mut symbol_table_lpair(),
                &fun::syntax::context::TypingContext::default(),
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
        let result = term_typed.compile(&mut state, core_lang::syntax::types::Ty::I64);

        let mut context1 = core_lang::syntax::TypingContext::default();
        context1.add_covar("a1", Ty::I64);
        let mut context2 = core_lang::syntax::TypingContext::default();
        context2.add_covar("a2", Ty::I64);
        let mut arguments = core_lang::syntax::arguments::Arguments::default();
        arguments.add_cons(core_lang::syntax::terms::XVar::covar(
            "a0",
            core_lang::syntax::types::Ty::I64,
        ));
        let expected = core_lang::syntax::terms::Mu::mu(
            "a0",
            core_lang::syntax::statements::Cut::new(
                core_lang::syntax::terms::XCase {
                    prdcns: Prd,
                    clauses: vec![
                        core_lang::syntax::terms::Clause {
                            prdcns: Prd,
                            xtor: "fst".to_owned(),
                            context: context1,
                            body: Rc::new(
                                core_lang::syntax::statements::Cut::new(
                                    core_lang::syntax::terms::Literal::new(1),
                                    core_lang::syntax::terms::XVar::covar(
                                        "a1",
                                        core_lang::syntax::types::Ty::I64,
                                    ),
                                    core_lang::syntax::types::Ty::I64,
                                )
                                .into(),
                            ),
                        },
                        core_lang::syntax::terms::Clause {
                            prdcns: Prd,
                            xtor: "snd".to_owned(),
                            context: context2,
                            body: Rc::new(
                                core_lang::syntax::statements::Cut::new(
                                    core_lang::syntax::terms::Literal::new(2),
                                    core_lang::syntax::terms::XVar::covar(
                                        "a2",
                                        core_lang::syntax::types::Ty::I64,
                                    ),
                                    core_lang::syntax::types::Ty::I64,
                                )
                                .into(),
                            ),
                        },
                    ],
                    ty: core_lang::syntax::types::Ty::Decl("LPair[i64, i64]".to_owned()),
                },
                core_lang::syntax::terms::Xtor {
                    prdcns: Cns,
                    id: "fst".to_owned(),
                    args: arguments,
                    ty: core_lang::syntax::types::Ty::Decl("LPair[i64, i64]".to_owned()),
                },
                core_lang::syntax::types::Ty::Decl("LPair[i64, i64]".to_owned()),
            ),
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_snd() {
        let term = parse_term!("new { fst => 1, snd => 2}.snd[i64, i64]");
        let term_typed = term
            .check(
                &mut symbol_table_lpair(),
                &fun::syntax::context::TypingContext::default(),
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
        let result = term_typed.compile(&mut state, core_lang::syntax::types::Ty::I64);

        let mut context1 = core_lang::syntax::TypingContext::default();
        context1.add_covar("a1", Ty::I64);
        let mut context2 = core_lang::syntax::TypingContext::default();
        context2.add_covar("a2", Ty::I64);
        let mut arguments = core_lang::syntax::arguments::Arguments::default();
        arguments.add_cons(core_lang::syntax::terms::XVar::covar(
            "a0",
            core_lang::syntax::types::Ty::I64,
        ));
        let expected = core_lang::syntax::terms::Mu::mu(
            "a0",
            core_lang::syntax::statements::Cut::new(
                core_lang::syntax::terms::XCase {
                    prdcns: Prd,
                    clauses: vec![
                        core_lang::syntax::terms::Clause {
                            prdcns: Prd,
                            xtor: "fst".to_owned(),
                            context: context1,
                            body: Rc::new(
                                core_lang::syntax::statements::Cut::new(
                                    core_lang::syntax::terms::Literal::new(1),
                                    core_lang::syntax::terms::XVar::covar(
                                        "a1",
                                        core_lang::syntax::types::Ty::I64,
                                    ),
                                    core_lang::syntax::types::Ty::I64,
                                )
                                .into(),
                            ),
                        },
                        core_lang::syntax::terms::Clause {
                            prdcns: Prd,
                            xtor: "snd".to_owned(),
                            context: context2,
                            body: Rc::new(
                                core_lang::syntax::statements::Cut::new(
                                    core_lang::syntax::terms::Literal::new(2),
                                    core_lang::syntax::terms::XVar {
                                        prdcns: Cns,
                                        var: "a2".to_owned(),
                                        ty: core_lang::syntax::types::Ty::I64,
                                    },
                                    core_lang::syntax::types::Ty::I64,
                                )
                                .into(),
                            ),
                        },
                    ],
                    ty: core_lang::syntax::types::Ty::Decl("LPair[i64, i64]".to_owned()),
                },
                core_lang::syntax::terms::Xtor {
                    prdcns: Cns,
                    id: "snd".to_owned(),
                    args: arguments,
                    ty: core_lang::syntax::types::Ty::Decl("LPair[i64, i64]".to_owned()),
                },
                core_lang::syntax::types::Ty::Decl("LPair[i64, i64]".to_owned()),
            ),
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }
}
