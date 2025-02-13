use crate::{
    compile::{CompileState, CompileWithCont},
    program::{compile_subst, compile_ty},
};
use core_lang::syntax::terms::Cns;
use fun::syntax::types::OptTyped;

impl CompileWithCont for fun::syntax::terms::Destructor {
    /// ```text
    /// 〚t.D(t_1, ...) 〛_{c} = 〚t〛_{D(〚t_1〛, ..., c)}
    /// ```
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
                self.destructee
                    .get_type()
                    .expect("Types should be annotated before translation"),
            ),
        }
        .into();

        // 〚t〛_{new_cont}
        self.destructee.compile_with_cont(new_cont, state)
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::{parse_term, test_common::symbol_table_lpair, typing::check::Check};

    use crate::compile::CompileWithCont;
    use core_lang::syntax::{
        context::Context,
        terms::{Cns, Prd},
        types::Ty,
    };
    use std::rc::Rc;

    #[test]
    fn compile_fst() {
        let term = parse_term!("cocase { Fst => 1, Snd => 2}.Fst[i64, i64]");
        let term_typed = term
            .check(
                &mut symbol_table_lpair(),
                &fun::syntax::context::TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::I64);
        let mut ctx1 = Context::new();
        ctx1.add_covar("a1", Ty::I64);
        let mut ctx2 = Context::new();
        ctx2.add_covar("a2", Ty::I64);
        let mut subst = core_lang::syntax::substitution::Substitution::default();
        subst.add_cons(core_lang::syntax::terms::XVar::covar(
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
                            xtor: "Fst".to_owned(),
                            context: ctx1,
                            rhs: Rc::new(
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
                            xtor: "Snd".to_owned(),
                            context: ctx2,
                            rhs: Rc::new(
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
                    id: "Fst".to_owned(),
                    args: subst,
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
        let term = parse_term!("cocase { Fst => 1, Snd => 2}.Snd[i64, i64]");
        let term_typed = term
            .check(
                &mut symbol_table_lpair(),
                &fun::syntax::context::TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::I64);
        let mut ctx1 = Context::new();
        ctx1.add_covar("a1", Ty::I64);
        let mut ctx2 = Context::new();
        ctx2.add_covar("a2", Ty::I64);
        let mut subst = core_lang::syntax::substitution::Substitution::default();
        subst.add_cons(core_lang::syntax::terms::XVar::covar(
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
                            xtor: "Fst".to_owned(),
                            context: ctx1,
                            rhs: Rc::new(
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
                            xtor: "Snd".to_owned(),
                            context: ctx2,
                            rhs: Rc::new(
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
                    id: "Snd".to_owned(),
                    args: subst,
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
